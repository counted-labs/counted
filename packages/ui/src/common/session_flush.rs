use std::sync::{Arc, OnceLock};

/// Writes the platform's cookie jar to disk now.
pub type SessionFlusher = Arc<dyn Fn() + Send + Sync + 'static>;

static FLUSHER: OnceLock<SessionFlusher> = OnceLock::new();

/// Installs the platform flusher. The mobile entry point calls this once before launch; every
/// other target leaves it unset, which makes [`flush_session`] a no-op — the browser owns its
/// own cookie jar. Same construction as `set_haptics_player`. Later calls are ignored.
pub fn set_session_flusher(flusher: SessionFlusher) {
    let _ = FLUSHER.set(flusher);
}

/// Called right after a sign-in, sign-up or sign-out returns. Mobile persists cookies on a timer,
/// and a process killed inside that window kept the account key on disk with no session beside
/// it — signed in by every local measure, refused by every request.
pub fn flush_session() {
    if let Some(flush) = FLUSHER.get() {
        flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flush_without_a_flusher_is_a_noop() {
        flush_session();
    }
}
