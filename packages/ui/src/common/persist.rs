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
        let Some(raw) = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item(key).ok().flatten())
        else {
            return T::default();
        };
        return match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                dioxus::logger::tracing::error!(
                    "counted: {key} did not parse ({} chars), starting empty: {e}",
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
/// cache silently frozen at whatever it last managed to save.
pub fn write_json<T: Serialize>(key: &str, label: &str, value: &T) {
    #[cfg(target_arch = "wasm32")]
    {
        let Ok(json) = serde_json::to_string(value) else {
            dioxus::logger::tracing::error!("counted: serialize {label} failed");
            return;
        };
        let stored = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .map(|s| s.set_item(key, &json));
        match stored {
            Some(Ok(())) => {}
            Some(Err(e)) => dioxus::logger::tracing::error!(
                "counted: write {label} failed ({} chars): {e:?}",
                json.len()
            ),
            None => dioxus::logger::tracing::error!("counted: no localStorage for {label}"),
        }
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
                let res = std::fs::write(&tmp, &json).and_then(|_| std::fs::rename(&tmp, &path));
                if let Err(e) = res {
                    let _ = std::fs::remove_file(&tmp);
                    dioxus::logger::tracing::error!("counted: write {} failed: {e}", path.display());
                }
            }
            Err(e) => dioxus::logger::tracing::error!("counted: serialize {label} failed: {e}"),
        }
    }
}
