//! This crate contains all shared UI for the workspace.

pub mod auth;
pub mod badges;
pub mod categories;
pub mod charts;
pub mod common;
pub mod conversion;
pub mod crypto;
pub mod decrypted;
pub mod demo;
pub mod entity_ids;
pub mod expenses;
pub mod friends;
pub mod help;
pub mod hybrid;
pub mod i18n;
pub mod icons;
pub mod legal;
pub mod not_found;
pub mod participants;
pub mod payment_methods;
pub mod payments;
pub mod preferences;
pub mod privacy;
pub mod project_history;
pub mod projects;
pub mod recurring;
pub mod rotation;
pub mod rotation_flow;
pub mod route;
pub mod unsubscribe;
pub mod welcome;

#[cfg(any(test, fuzzing))]
pub mod fuzz;
#[cfg(test)]
mod time_golden;
