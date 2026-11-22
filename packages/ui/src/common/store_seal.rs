//! The mobile store sealed at rest (docs/plans/security-3.3.0.md §4). Every store file under the
//! data dir is `CNTS` + version + nonce + XChaCha20-Poly1305 under one store key, which the platform
//! keeps wrapped by the Android Keystore or in the iOS Keychain. The associated data names the store,
//! so one store's file cannot stand in for another's.
//!
//! Desktop and test builds install no provider and stay plain. A sealed file on disk proves a key
//! existed: the provider is never allowed to create one then.

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305,
};
use std::sync::{Arc, RwLock};

const MAGIC: &[u8] = b"CNTS\x01";
const NONCE: usize = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StoreKey {
    /// No sealing on this build, or the platform offers none while nothing on disk is sealed.
    Plain,
    /// `created`: the key was made on this launch, the one launch that may adopt plain stores.
    Ready { key: [u8; 32], created: bool },
    /// The key exists but cannot be used now (Keystore error, iOS before the first unlock).
    Locked,
    /// The platform says the key is gone while sealed data remains.
    Lost,
}

pub enum KeyRequest {
    Open { may_create: bool },
    Reset,
}

pub type KeyProvider = Arc<dyn Fn(KeyRequest) -> StoreKey + Send + Sync>;

static STATE: RwLock<StoreKey> = RwLock::new(StoreKey::Plain);
static PROVIDER: RwLock<Option<KeyProvider>> = RwLock::new(None);
static ON_UNLOCK: RwLock<Vec<Arc<dyn Fn() + Send + Sync>>> = RwLock::new(Vec::new());

fn aad(name: &str) -> Vec<u8> {
    format!("counted/store/v1/{name}").into_bytes()
}

pub fn is_sealed(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC)
}

pub fn seal(key: &[u8; 32], name: &str, plaintext: &[u8]) -> Option<Vec<u8>> {
    let mut nonce = [0u8; NONCE];
    getrandom::getrandom(&mut nonce).ok()?;
    let ct = XChaCha20Poly1305::new(key.into()).encrypt(&nonce.into(), Payload { msg: plaintext, aad: &aad(name) }).ok()?;
    Some([MAGIC, &nonce, &ct].concat())
}

/// `None` for a file that is not sealed, sealed under another key or another store's name, or
/// altered.
pub fn open(key: &[u8; 32], name: &str, bytes: &[u8]) -> Option<Vec<u8>> {
    let rest = bytes.strip_prefix(MAGIC)?;
    if rest.len() < NONCE {
        return None;
    }
    let (nonce, ct) = rest.split_at(NONCE);
    let nonce: [u8; NONCE] = nonce.try_into().ok()?;
    XChaCha20Poly1305::new(key.into()).decrypt(&nonce.into(), Payload { msg: ct, aad: &aad(name) }).ok()
}

pub fn state() -> StoreKey {
    *STATE.read().unwrap_or_else(|e| e.into_inner())
}

fn set_state(key: StoreKey) {
    *STATE.write().unwrap_or_else(|e| e.into_inner()) = key;
}

/// Whether a store may be read, written or deleted. False while locked or lost, so nothing an
/// empty session does reaches the files the key would open.
pub fn usable() -> bool {
    matches!(state(), StoreKey::Plain | StoreKey::Ready { .. })
}

/// Installed by `mobile` before the first store read; asks the platform for the key at once.
pub fn install(provider: KeyProvider) -> StoreKey {
    *PROVIDER.write().unwrap_or_else(|e| e.into_inner()) = Some(provider);
    resolve()
}

/// Called again when the locked screen asks to retry.
pub fn retry() -> StoreKey {
    resolve()
}

/// Run once the store opens after a locked launch, so what was read empty is read again.
pub fn on_unlock(hook: Arc<dyn Fn() + Send + Sync>) {
    ON_UNLOCK.write().unwrap_or_else(|e| e.into_inner()).push(hook);
}

/// Starts over: every sealed file is set aside as `.lost` (never read again, never deleted here),
/// the platform drops its key, and a new one is made. Used by erase device and by the reset the
/// locked screen offers.
pub fn reset() -> StoreKey {
    #[cfg(not(target_arch = "wasm32"))]
    for path in sealed_files(&super::persist::data_dir()) {
        let mut lost = path.clone().into_os_string();
        lost.push(".lost");
        let _ = std::fs::rename(&path, lost);
    }
    if let Some(provider) = provider() {
        provider(KeyRequest::Reset);
    }
    resolve()
}

/// Erase device: every sealed file goes, the platform drops its key, and a new one is made, so
/// nothing left on the phone opens under a key that still exists.
pub fn erase() -> StoreKey {
    #[cfg(not(target_arch = "wasm32"))]
    for path in sealed_files(&super::persist::data_dir()) {
        let _ = std::fs::remove_file(path);
    }
    if let Some(provider) = provider() {
        provider(KeyRequest::Reset);
    }
    resolve()
}

#[cfg(test)]
pub(crate) fn uninstall() {
    *PROVIDER.write().unwrap_or_else(|e| e.into_inner()) = None;
    set_state(StoreKey::Plain);
}

fn provider() -> Option<KeyProvider> {
    PROVIDER.read().unwrap_or_else(|e| e.into_inner()).clone()
}

fn resolve() -> StoreKey {
    let was_usable = usable();
    #[cfg(not(target_arch = "wasm32"))]
    let sealed_on_disk = !sealed_files(&super::persist::data_dir()).is_empty();
    #[cfg(target_arch = "wasm32")]
    let sealed_on_disk = false;
    let key = match provider() {
        Some(provider) => match provider(KeyRequest::Open { may_create: !sealed_on_disk }) {
            StoreKey::Ready { created: true, .. } if sealed_on_disk => StoreKey::Lost,
            StoreKey::Plain if sealed_on_disk => StoreKey::Lost,
            other => other,
        },
        None if sealed_on_disk => StoreKey::Locked,
        None => StoreKey::Plain,
    };
    set_state(key);
    #[cfg(not(target_arch = "wasm32"))]
    if let StoreKey::Ready { key, created: true } = key {
        seal_leftovers(&super::persist::data_dir(), &key);
    }
    if !was_usable && usable() {
        for hook in ON_UNLOCK.read().unwrap_or_else(|e| e.into_inner()).iter() {
            hook();
        }
    }
    key
}

/// Every file directly under `dir` that starts sealed, `.lost` ones excepted.
#[cfg(not(target_arch = "wasm32"))]
fn sealed_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_none_or(|x| x != "lost"))
        .filter(|p| {
            let mut head = [0u8; 5];
            std::fs::File::open(p).and_then(|mut f| std::io::Read::read_exact(&mut f, &mut head)).is_ok() && is_sealed(&head)
        })
        .collect()
}

/// On the launch that made the key: an interrupted write's tmp file is dropped, and a plain
/// quarantined store, which may hold the only copy of a project key, is sealed in place.
#[cfg(not(target_arch = "wasm32"))]
fn seal_leftovers(dir: &std::path::Path, key: &[u8; 32]) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for path in entries.filter_map(|e| e.ok().map(|e| e.path())) {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        if name.ends_with(".json.tmp") {
            let _ = std::fs::remove_file(&path);
        } else if let Some(store) = name.strip_suffix(".json.corrupt") {
            let Ok(bytes) = std::fs::read(&path) else { continue };
            if !is_sealed(&bytes) {
                if let Some(sealed) = seal(key, &format!("{store}.corrupt"), &bytes) {
                    let _ = std::fs::write(&path, sealed);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; 32] = [7; 32];

    #[test]
    fn a_sealed_store_opens_under_its_own_name_only() {
        let sealed = seal(&KEY, "counted_local_storage", b"{\"projects\":[]}").unwrap();
        assert!(is_sealed(&sealed));
        assert_eq!(open(&KEY, "counted_local_storage", &sealed).as_deref(), Some(&b"{\"projects\":[]}"[..]));
        assert_eq!(open(&KEY, "counted_offline_queue", &sealed), None, "another store's name");
        assert_eq!(open(&[8; 32], "counted_local_storage", &sealed), None, "another key");
    }

    #[test]
    fn an_altered_or_short_or_plain_file_does_not_open() {
        let mut sealed = seal(&KEY, "s", b"{}").unwrap();
        let last = sealed.len() - 1;
        sealed[last] ^= 1;
        assert_eq!(open(&KEY, "s", &sealed), None);
        assert_eq!(open(&KEY, "s", b"CNTS\x01short"), None);
        assert_eq!(open(&KEY, "s", b"{}"), None);
        assert!(!is_sealed(b"{}"));
    }

    #[test]
    fn two_seals_of_the_same_store_differ() {
        assert_ne!(seal(&KEY, "s", b"{}"), seal(&KEY, "s", b"{}"));
    }

    /// The provider stands in for the Keystore: it creates only when allowed, and records what it
    /// was asked.
    #[cfg(not(target_arch = "wasm32"))]
    fn with_platform<T>(answer: fn(bool) -> StoreKey, f: impl FnOnce(&std::path::Path) -> T) -> T {
        crate::common::test_fixtures::with_data_dir("store_seal", |dir| {
            let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                install(Arc::new(move |request| match request {
                    KeyRequest::Open { may_create } => answer(may_create),
                    KeyRequest::Reset => StoreKey::Plain,
                }));
                f(dir)
            }));
            *PROVIDER.write().unwrap() = None;
            ON_UNLOCK.write().unwrap().clear();
            set_state(StoreKey::Plain);
            out.unwrap()
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_when_allowed(may_create: bool) -> StoreKey {
        match may_create {
            true => StoreKey::Ready { key: KEY, created: true },
            false => StoreKey::Ready { key: KEY, created: false },
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn no_key_is_made_while_a_sealed_file_is_on_disk() {
        let state = with_platform(|_| StoreKey::Ready { key: [9; 32], created: true }, |dir| {
            std::fs::write(dir.join("counted_offline_queue.json"), seal(&KEY, "counted_offline_queue", b"[]").unwrap()).unwrap();
            resolve()
        });
        assert_eq!(state, StoreKey::Lost);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_launch_that_makes_the_key_seals_a_plain_quarantine_and_drops_tmp_files() {
        let (corrupt, tmp_left) = with_platform(create_when_allowed, |dir| {
            std::fs::write(dir.join("counted_local_storage.json.corrupt"), b"{\"account_key\":\"k\"").unwrap();
            std::fs::write(dir.join("counted_local_storage.json.tmp"), b"{").unwrap();
            resolve();
            (std::fs::read(dir.join("counted_local_storage.json.corrupt")).unwrap(), dir.join("counted_local_storage.json.tmp").exists())
        });
        assert_eq!(open(&KEY, "counted_local_storage.corrupt", &corrupt).as_deref(), Some(&b"{\"account_key\":\"k\""[..]));
        assert!(!tmp_left);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_plain_store_is_sealed_by_its_first_read_then_a_planted_one_is_set_aside() {
        use crate::common::persist::{file_for, read_json};
        let (migrated, sealed_after, planted, aside) = with_platform(create_when_allowed, |dir| {
            std::fs::write(file_for("s"), b"{\"a\":1}").unwrap();
            resolve();
            let migrated: serde_json::Value = read_json("s");
            let sealed_after = std::fs::read(file_for("s")).unwrap();
            assert_eq!(resolve(), StoreKey::Ready { key: KEY, created: false }, "the next launch");
            std::fs::write(file_for("s"), b"{\"a\":2}").unwrap();
            let planted: serde_json::Value = read_json("s");
            (migrated, sealed_after, planted, dir.join("s.json.corrupt").exists())
        });
        assert_eq!(migrated, serde_json::json!({ "a": 1 }));
        assert_eq!(open(&KEY, "s", &sealed_after).as_deref(), Some(&b"{\"a\":1}"[..]));
        assert_eq!(planted, serde_json::Value::Null, "not believed");
        assert!(aside);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn reset_sets_sealed_files_aside_and_makes_a_new_key() {
        let (state, aside, left) = with_platform(create_when_allowed, |dir| {
            std::fs::write(dir.join("counted_local_storage.json"), seal(&[9; 32], "counted_local_storage", b"{}").unwrap()).unwrap();
            assert_eq!(resolve(), StoreKey::Ready { key: KEY, created: false });
            (reset(), dir.join("counted_local_storage.json.lost").exists(), dir.join("counted_local_storage.json").exists())
        });
        assert_eq!(state, StoreKey::Ready { key: KEY, created: true });
        assert!(aside && !left);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_store_that_opens_after_a_locked_launch_is_read_again() {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        static UNLOCKED: AtomicBool = AtomicBool::new(false);
        static READS: AtomicUsize = AtomicUsize::new(0);
        let reads = with_platform(
            |_| match UNLOCKED.load(Ordering::SeqCst) {
                true => StoreKey::Ready { key: KEY, created: false },
                false => StoreKey::Locked,
            },
            |_| {
                on_unlock(Arc::new(|| {
                    READS.fetch_add(1, Ordering::SeqCst);
                }));
                assert_eq!(retry(), StoreKey::Locked);
                UNLOCKED.store(true, Ordering::SeqCst);
                retry();
                retry();
                READS.load(Ordering::SeqCst)
            },
        );
        assert_eq!(reads, 1, "once, on the launch's first unlock");
    }
}
