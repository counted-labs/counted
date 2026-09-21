//! All the scan logic, so `ExpensesTab` stays a dumb renderer of two signals and a callback.

use dioxus::prelude::*;

use crate::common::{
    capture_and_scan, haptic, scan_error_key, scanner_available, Haptic, ScanError,
};
use crate::expenses::helpers::scan_prefill::{prefill_from_scan, ScanPrefill};

#[derive(Clone, Copy)]
pub struct ReceiptScan {
    /// What the camera produced, cleared when the modal closes.
    pub prefill: Signal<Option<ScanPrefill>>,
    /// A message key, or `None`. Rendered through the existing `Toast`.
    pub error: Signal<Option<&'static str>>,
    busy: Signal<bool>,
    pub start: Callback<()>,
}

impl ReceiptScan {
    /// Drives the button's spinner and its disabled state.
    pub fn busy(&self) -> bool {
        (self.busy)()
    }

    /// False on web, desktop and the server binary, where nothing installs a scanner.
    pub fn available(&self) -> bool {
        scanner_available()
    }
}

/// `open_modal` is raised on success, so a scan lands the user directly in the prefilled form.
pub fn use_receipt_scan(mut open_modal: Signal<bool>) -> ReceiptScan {
    let mut prefill: Signal<Option<ScanPrefill>> = use_signal(|| None);
    let mut error: Signal<Option<&'static str>> = use_signal(|| None);
    let mut busy = use_signal(|| false);

    let start = use_callback(move |_| {
        // The camera takes a moment to appear and the button stays visible behind it; without the
        // guard a second tap starts a second capture and a second inference thread.
        if busy() {
            return;
        }
        busy.set(true);
        error.set(None);
        haptic(Haptic::Light);

        spawn(async move {
            let outcome = capture_and_scan().await;
            busy.set(false);
            match outcome {
                Ok(fields) => {
                    prefill.set(Some(prefill_from_scan(&fields)));
                    open_modal.set(true);
                    haptic(Haptic::Success);
                }
                // Backing out of the camera is a decision, not a failure. Nothing is shown, and
                // `Unsupported` cannot be reached from a button that only renders when a scanner
                // is installed.
                Err(ScanError::Cancelled) | Err(ScanError::Unsupported) => {}
                Err(e) => {
                    error.set(Some(scan_error_key(e)));
                    haptic(Haptic::Error);
                }
            }
        });
    });

    ReceiptScan { prefill, error, busy, start }
}

#[cfg(test)]
mod tests;
