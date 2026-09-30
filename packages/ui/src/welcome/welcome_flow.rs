use dioxus::prelude::*;

use crate::common::{scanner_available, update_ls, LocalStorageState};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WelcomeStep {
    Promise,
    Link,
    Private,
    Scan,
}

const WITH_SCAN: &[WelcomeStep] = &[
    WelcomeStep::Promise,
    WelcomeStep::Link,
    WelcomeStep::Private,
    WelcomeStep::Scan,
];
const WITHOUT_SCAN: &[WelcomeStep] = &[WelcomeStep::Promise, WelcomeStep::Link, WelcomeStep::Private];

/// The scan step is dropped where no scanner is installed: `is_mobile()` is a UA sniff on web, so
/// a phone browser gets this flow too and must not be shown a feature it cannot use.
pub fn welcome_steps(scanner: bool) -> &'static [WelcomeStep] {
    if scanner {
        WITH_SCAN
    } else {
        WITHOUT_SCAN
    }
}

pub fn next_index(index: usize, total: usize) -> usize {
    (index + 1).min(total.saturating_sub(1))
}

pub fn last_index(total: usize) -> usize {
    total.saturating_sub(1)
}

#[derive(Clone, Copy, PartialEq)]
pub struct WelcomeFlow {
    steps: &'static [WelcomeStep],
    index: Signal<usize>,
    ls: Signal<LocalStorageState>,
}

impl WelcomeFlow {
    pub fn step(&self) -> WelcomeStep {
        self.steps[(self.index)()]
    }

    pub fn position(&self) -> usize {
        (self.index)() + 1
    }

    pub fn total(&self) -> usize {
        self.steps.len()
    }

    pub fn is_last(&self) -> bool {
        (self.index)() == last_index(self.total())
    }

    pub fn next(mut self) {
        let next = next_index((self.index)(), self.total());
        self.index.set(next);
    }

    pub fn skip(mut self) {
        self.index.set(last_index(self.total()));
    }

    pub fn finish(self) {
        update_ls(self.ls, |state| state.onboarding_seen = true);
    }
}

pub fn use_welcome_flow() -> WelcomeFlow {
    let ls = use_context::<Signal<LocalStorageState>>();
    let index = use_signal(|| 0);
    WelcomeFlow { steps: welcome_steps(scanner_available()), index, ls }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scan_step_is_last_and_only_where_a_scanner_exists() {
        assert_eq!(welcome_steps(true).last(), Some(&WelcomeStep::Scan));
        assert!(!welcome_steps(false).contains(&WelcomeStep::Scan));
        assert_eq!(welcome_steps(false).len(), 3);
    }

    #[test]
    fn next_stops_on_the_last_step() {
        assert_eq!(next_index(0, 4), 1);
        assert_eq!(next_index(3, 4), 3);
        assert_eq!(next_index(2, 3), 2);
    }

    #[test]
    fn skip_lands_on_the_last_step() {
        assert_eq!(last_index(4), 3);
        assert_eq!(last_index(3), 2);
    }
}
