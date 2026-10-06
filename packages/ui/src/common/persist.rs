//! The one place that knows how a store is persisted: `localStorage` on web, a JSON file
//! under the data dir on native. `local_storage` and `offline_queue` both go through here.

use serde::{de::DeserializeOwned, Serialize};

/// `COUNTED_DATA_DIR`, else `HOME`, else the working directory. Also used by the export
/// helper — every native path in the app resolves from here.
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
fn web_storage() -> Option<web_sys::Storage> {
    match is_demo() {
        true => session_storage(),
        false => web_sys::window()?.local_storage().ok().flatten(),
    }
}

/// Reads a store, falling back to `T::default()` when there is nothing to read.
///
/// **A parse failure is not the same as an empty store, and used to be indistinguishable from one.**
/// The default was returned silently, and the next `write_json` — which happens constantly — made
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
        return match serde_json::from_str(&raw) {
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
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = file_for(key);
        let Ok(raw) = std::fs::read_to_string(&path) else {
            return T::default();
        };
        match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                // Renamed rather than overwritten: whatever is in there may be the only copy of a
                // project key, and the very next write would otherwise destroy it.
                let quarantine = path.with_extension("json.corrupt");
                let kept = std::fs::rename(&path, &quarantine).is_ok();
                dioxus::logger::tracing::error!(
                    "counted: {} did not parse, starting empty (kept a copy: {kept}): {e}",
                    path.display()
                );
                T::default()
            }
        }
    }
}

/// `label` names the store in the serialization error log ("local storage" / "offline queue").
///
/// A failed write is reported on every target. On web that matters more than it looks: localStorage
/// is a hard ~5 MB per origin and `set_item` throws `QuotaExceededError` when it is full, which this
/// used to swallow — leaving a full store indistinguishable from a healthy one, and the offline
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
        return false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = file_for(key);
        match serde_json::to_string(value) {
            Ok(json) => {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                // Write to a sibling tmp file and rename: an interrupted write leaves the
                // previous contents intact instead of a truncated file.
                let tmp = path.with_extension("json.tmp");
                // `sync_all` before the rename: otherwise a power loss can persist the rename ahead
                // of the data and leave a zero-length store.
                let res = std::fs::File::create(&tmp)
                    .and_then(|mut f| {
                        std::io::Write::write_all(&mut f, json.as_bytes())?;
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
            Err(e) => {
                dioxus::logger::tracing::error!("counted: serialize {label} failed: {e}");
                false
            }
        }
    }
}
