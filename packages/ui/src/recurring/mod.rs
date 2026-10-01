//! Recurring expenses, client side. The server can neither read nor write an occurrence, so this
//! module computes them and the server commits each batch exactly once. See
//! docs/plans/recurring-expenses.md.

pub mod actions;
pub mod dialog;
pub mod edit_rule_modal;
pub mod labels;
pub mod model;
pub mod recurring_page;
pub mod repeat_picker;
pub mod requests;
pub mod schedule;
pub mod materializer;
pub mod split;
pub mod store;
pub mod view;
