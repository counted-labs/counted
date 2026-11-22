//! The one place that knows how a store is persisted: `localStorage` on web, a JSON file
//! under the data dir on native. `local_storage` and `offline_queue` both go through here.

use serde::{de::DeserializeOwned, Serialize};

/// `COUNTED_DATA_DIR`, else `HOME`, else the working directory. Also used by the export
/// helper - every native path in the app resolves from here.
#[cfg(not(target_arch = "wasm32"))]
pub fn data_dir() -> std::path::PathBuf {
    let dir = std::env::var("COUNTED_DATA_DIR")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(dir)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn file_for(key: &str) -> std::path::PathBuf {
    data_dir().join(format!("{key}.json"))
}

#[cfg(target_arch = "wasm32")]
const DEMO_FLAG: &str = "counted_demo";
#[cfg(target_arch = "wasm32")]
const DEMO_PATH: &str = "/demo";

#[cfg(target_arch = "wasm32")]
fn session_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.session_storage().ok().flatten()
}

/// The `/demo` sandbox. A tab in it keeps every store in sessionStorage under the same keys, so the
/// visitor's own projects, keys and account are neither read nor written, and closing the tab
/// erases the demo. Always false off the web: the demo is web-only.
pub fn is_demo() -> bool {
    #[cfg(target_arch = "wasm32")]
    return session_storage().is_some_and(|s| s.get_item(DEMO_FLAG).ok().flatten().is_some());
    #[cfg(not(target_arch = "wasm32"))]
    false
}

/// Must run before launch: the first render already reads the stores.
#[cfg(target_arch = "wasm32")]
pub fn enter_demo_if_requested() {
    let on_demo = web_sys::window().and_then(|w| w.location().pathname().ok()).is_some_and(|p| p == DEMO_PATH);
    if let (true, Some(s)) = (on_demo, session_storage()) {
        let _ = s.set_item(DEMO_FLAG, "1");
    }
}

/// Wipes the sandbox, flag included. The caller reloads, so every context re-reads the real stores.
#[cfg(target_arch = "wasm32")]
pub fn clear_demo() {
    if let Some(s) = session_storage() {
        let _ = s.clear();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn web_storage() -> Option<web_sys::Storage> {
    match is_demo() {
        true => session_storage(),
        false => web_sys::window()?.local_storage().ok().flatten(),
    }
}

/// Reads a store, falling back to `T::default()` when there is nothing to read.
///
/// **A parse failure is not the same as an empty store, and used to be indistinguishable from one.**
/// The default was returned silently, and the next `write_json` - which happens constantly - made
/// that loss permanent. Since the E2EE keys share this blob with the cached DTOs, any non-additive
/// change to `ProjectDto`/`User`/`Expense`/`Payment` could take key material with it, with no log
/// and nothing to recover from. So: log it, and on native keep the bytes.
pub fn read_json<T: DeserializeOwned + Default>(key: &str) -> T {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_storage();
        let Some(raw) = storage.as_ref().and_then(|s| s.get_item(key).ok().flatten()) else {
            return T::default();
        };
        match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                // Moved aside like native's rename, so the next write cannot destroy what may be the
                // only copy of a project key. A full quota makes the copy fail; that is logged.
                let quarantine = format!("{key}.corrupt");
                let kept = storage.as_ref().is_some_and(|s| {
                    s.set_item(&quarantine, &raw).is_ok() && s.remove_item(key).is_ok()
                });
                dioxus::logger::tracing::error!(
                    "counted: {key} did not parse ({} chars), starting empty (kept a copy: {kept}): {e}",
                    raw.len()
                );
                T::default()
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Bytes, not a string: a sealed file is not UTF-8, and a read that failed on it used to
        // look like an empty store that the next write made permanent.
        let path = file_for(key);
        let Ok(bytes) = std::fs::read(&path) else {
            return T::default();
        };
        let state = store_seal::state();
        let migrate = matches!(state, StoreKey::Ready { created: true, .. }) && !store_seal::is_sealed(&bytes);
        match loaded(state, key, &bytes) {
            Loaded::Skip => T::default(),
            Loaded::Quarantine(why) => {
                quarantine(state, key, &bytes, why);
                T::default()
            }
            Loaded::Json(json) => match serde_json::from_slice(&json) {
                Ok(v) => {
                    if migrate {
                        write_bytes(state, key, &json);
                    }
                    v
                }
                Err(e) => {
                    quarantine(state, key, &bytes, &e.to_string());
                    T::default()
                }
            },
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
use super::store_seal::{self, StoreKey};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, PartialEq)]
enum Loaded {
    Json(Vec<u8>),
    Quarantine(&'static str),
    Skip,
}

/// What a store file's bytes are worth under the key state: a plain store is adopted only on the
/// launch that made the key, so a plain file planted after that is set aside, not believed.
#[cfg(not(target_arch = "wasm32"))]
fn loaded(state: StoreKey, name: &str, bytes: &[u8]) -> Loaded {
    match state {
        StoreKey::Locked | StoreKey::Lost => Loaded::Skip,
        StoreKey::Plain if store_seal::is_sealed(bytes) => Loaded::Skip,
        StoreKey::Plain => Loaded::Json(bytes.to_vec()),
        StoreKey::Ready { key, .. } if store_seal::is_sealed(bytes) => match store_seal::open(&key, name, bytes) {
            Some(json) => Loaded::Json(json),
            None => Loaded::Quarantine("a sealed store did not open"),
        },
        StoreKey::Ready { created: true, .. } => Loaded::Json(bytes.to_vec()),
        StoreKey::Ready { created: false, .. } => Loaded::Quarantine("a plain store where every store is sealed"),
    }
}

/// Moved aside rather than overwritten: whatever is in there may be the only copy of a project key,
/// and the very next write would otherwise destroy it. A plain copy is sealed when a key is ready.
#[cfg(not(target_arch = "wasm32"))]
fn quarantine(state: StoreKey, name: &str, bytes: &[u8], why: &str) {
    let path = file_for(name);
    let aside = path.with_extension("json.corrupt");
    let kept = match state {
        StoreKey::Ready { key, .. } if !store_seal::is_sealed(bytes) => store_seal::seal(&key, &format!("{name}.corrupt"), bytes)
            .is_some_and(|sealed| std::fs::write(&aside, sealed).is_ok() && std::fs::remove_file(&path).is_ok()),
        _ => std::fs::rename(&path, &aside).is_ok(),
    };
    dioxus::logger::tracing::error!("counted: {} set aside, starting empty (kept a copy: {kept}): {why}", path.display());
}

/// Deletes the copy [`read_json`] set aside. It can hold every project key and the account key, so
/// it goes with the session instead of staying for good. Nothing is deleted while the store is
/// locked: an empty session must not reach the files the key would open.
pub fn remove_quarantine(key: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(s) = web_storage() {
        let _ = s.remove_item(&format!("{key}.corrupt"));
    }
    #[cfg(not(target_arch = "wasm32"))]
    if store_seal::usable() {
        let _ = std::fs::remove_file(file_for(key).with_extension("json.corrupt"));
    }
}

pub fn remove_store(key: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(s) = web_storage() {
        let _ = s.remove_item(key);
    }
    #[cfg(not(target_arch = "wasm32"))]
    if store_seal::usable() {
        let _ = std::fs::remove_file(file_for(key));
    }
    remove_quarantine(key);
}

/// `label` names the store in the serialization error log ("local storage" / "offline queue").
///
/// A failed write is reported on every target. On web that matters more than it looks: localStorage
/// is a hard ~5 MB per origin and `set_item` throws `QuotaExceededError` when it is full, which this
/// used to swallow - leaving a full store indistinguishable from a healthy one, and the offline
/// cache silently frozen at whatever it last managed to save. True when the value reached storage.
pub fn write_json<T: Serialize>(key: &str, label: &str, value: &T) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let Ok(json) = serde_json::to_string(value) else {
            dioxus::logger::tracing::error!("counted: serialize {label} failed");
            return false;
        };
        let stored = web_storage().map(|s| s.set_item(key, &json));
        match stored {
            Some(Ok(())) => return true,
            Some(Err(e)) => dioxus::logger::tracing::error!(
                "counted: write {label} failed ({} chars): {e:?}",
                json.len()
            ),
            None => dioxus::logger::tracing::error!("counted: no localStorage for {label}"),
        }
        false
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let state = store_seal::state();
        if !store_seal::usable() {
            dioxus::logger::tracing::error!("counted: {label} not written, the store is locked");
            return false;
        }
        match serde_json::to_vec(value) {
            Ok(json) => write_bytes(state, key, &json),
            Err(e) => {
                dioxus::logger::tracing::error!("counted: serialize {label} failed: {e}");
                false
            }
        }
    }
}

/// A store that is not JSON the app parses (the mobile cookie jar), under the same rules as
/// [`read_json`]: opened when sealed, adopted and sealed on the launch that made the key, set aside
/// when it does not open, nothing while locked.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_raw(name: &str) -> Option<Vec<u8>> {
    let bytes = std::fs::read(file_for(name)).ok()?;
    let state = store_seal::state();
    match loaded(state, name, &bytes) {
        Loaded::Skip => None,
        Loaded::Quarantine(why) => {
            quarantine(state, name, &bytes, why);
            None
        }
        Loaded::Json(plain) => {
            if matches!(state, StoreKey::Ready { created: true, .. }) && !store_seal::is_sealed(&bytes) {
                write_bytes(state, name, &plain);
            }
            Some(plain)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn write_raw(name: &str, bytes: &[u8]) -> bool {
    store_seal::usable() && write_bytes(store_seal::state(), name, bytes)
}

/// Whether a write can reach storage at all; false while the mobile store is locked, so a caller's
/// fallback write is not tried either.
pub fn writable() -> bool {
    #[cfg(target_arch = "wasm32")]
    return true;
    #[cfg(not(target_arch = "wasm32"))]
    store_seal::usable()
}

/// Sealed under the store key when one is ready, plain otherwise.
#[cfg(not(target_arch = "wasm32"))]
fn stored_bytes(state: StoreKey, name: &str, json: &[u8]) -> Option<Vec<u8>> {
    match state {
        StoreKey::Ready { key, .. } => store_seal::seal(&key, name, json),
        StoreKey::Plain => Some(json.to_vec()),
        StoreKey::Locked | StoreKey::Lost => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_bytes(state: StoreKey, name: &str, json: &[u8]) -> bool {
    let path = file_for(name);
    let Some(bytes) = stored_bytes(state, name, json) else {
        return false;
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Write to a sibling tmp file and rename: an interrupted write leaves the previous contents
    // intact instead of a truncated file. `sync_all` before the rename: otherwise a power loss can
    // persist the rename ahead of the data and leave a zero-length store.
    let tmp = path.with_extension("json.tmp");
    let res = std::fs::File::create(&tmp)
        .and_then(|mut f| {
            std::io::Write::write_all(&mut f, &bytes)?;
            f.sync_all()
        })
        .and_then(|_| std::fs::rename(&tmp, &path));
    match res {
        Ok(()) => true,
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            dioxus::logger::tracing::error!("counted: write {} failed: {e}", path.display());
            false
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod sealed_tests {
    use super::*;

    const KEY: [u8; 32] = [7; 32];
    const READY: StoreKey = StoreKey::Ready { key: KEY, created: false };
    const CREATED: StoreKey = StoreKey::Ready { key: KEY, created: true };

    #[test]
    fn a_sealed_store_reads_back_under_its_key() {
        let sealed = stored_bytes(READY, "s", b"{\"a\":1}").unwrap();
        assert!(store_seal::is_sealed(&sealed));
        assert_eq!(loaded(READY, "s", &sealed), Loaded::Json(b"{\"a\":1}".to_vec()));
    }

    #[test]
    fn a_plain_store_is_adopted_only_on_the_launch_that_made_the_key() {
        assert_eq!(loaded(CREATED, "s", b"{}"), Loaded::Json(b"{}".to_vec()));
        assert!(matches!(loaded(READY, "s", b"{}"), Loaded::Quarantine(_)), "planted after the store was sealed");
    }

    #[test]
    fn a_sealed_store_that_does_not_open_is_set_aside() {
        let mut sealed = stored_bytes(READY, "s", b"{}").unwrap();
        let last = sealed.len() - 1;
        sealed[last] ^= 1;
        assert!(matches!(loaded(READY, "s", &sealed), Loaded::Quarantine(_)));
        let other_store = stored_bytes(READY, "other", b"{}").unwrap();
        assert!(matches!(loaded(READY, "s", &other_store), Loaded::Quarantine(_)), "another store's file");
    }

    #[test]
    fn nothing_is_read_or_written_while_locked_or_lost() {
        let sealed = stored_bytes(READY, "s", b"{}").unwrap();
        for state in [StoreKey::Locked, StoreKey::Lost] {
            assert_eq!(loaded(state, "s", &sealed), Loaded::Skip);
            assert_eq!(loaded(state, "s", b"{}"), Loaded::Skip);
            assert_eq!(stored_bytes(state, "s", b"{}"), None);
        }
    }

    #[test]
    fn a_plain_build_never_reads_a_sealed_store_and_writes_plain() {
        let sealed = stored_bytes(READY, "s", b"{}").unwrap();
        assert_eq!(loaded(StoreKey::Plain, "s", &sealed), Loaded::Skip);
        assert_eq!(stored_bytes(StoreKey::Plain, "s", b"{}"), Some(b"{}".to_vec()));
    }
}
