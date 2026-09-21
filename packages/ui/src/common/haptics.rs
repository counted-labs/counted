use std::sync::{Arc, OnceLock};

/// What a haptic means, not what it feels like — the entry point maps these onto each platform's
/// own vocabulary (UIKit's impact/notification generators, Android's `HapticFeedbackConstants`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Haptic {
    /// A selection changed: a dock tab, a menu opening, the FAB.
    Light,
    /// A gesture crossed a threshold or committed: a row swipe arming, a long press firing.
    Medium,
    Success,
    Warning,
    Error,
}

pub type HapticsPlayer = Arc<dyn Fn(Haptic) + Send + Sync + 'static>;

static PLAYER: OnceLock<HapticsPlayer> = OnceLock::new();

/// Installs the platform player. The mobile entry point calls this once before launch; every other
/// target leaves it unset, which makes [`haptic`] a no-op. Later calls are ignored.
pub fn set_haptics_player(player: HapticsPlayer) {
    let _ = PLAYER.set(player);
}

/// Plays `kind`, or does nothing when no player is installed.
///
/// A process-global rather than a Dioxus context like `NativeClipboardReader`, because every call
/// site is an event handler well down the tree — a swipe passing its threshold, a toast mounting, a
/// long press opening a menu — and a context would need a `use_context` hook in each of their
/// components just to reach it. Nothing request- or session-specific is stored: the only value is a
/// function the entry point owns for the life of the process, so this is not the kind of state
/// DOCUMENTATION.md §3.6 forbids (which is about the server binary in any case).
pub fn haptic(kind: Haptic) {
    if let Some(play) = PLAYER.get() {
        play(kind);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole point of the `OnceLock`: a UI crate compiled for web, desktop and the SSR server
    /// binary installs nothing, and every call site must still be safe to run.
    #[test]
    fn haptic_without_a_player_is_a_noop() {
        haptic(Haptic::Light);
        haptic(Haptic::Error);
    }
}
