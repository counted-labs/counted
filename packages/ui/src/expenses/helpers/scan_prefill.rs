//! A scan result turned into the props `AddExpenseModal` already takes.

use crate::categories::infer_chart_category;
use crate::common::ScanFields;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScanPrefill {
    pub name: Option<String>,
    pub amount: Option<f64>,
    /// `%Y-%m-%d`, the shape `ExpenseForm`'s date signal holds.
    pub date: Option<String>,
    /// A `CHART_CATEGORIES` id.
    pub category: Option<String>,
    /// A key to show above the amount field, or `None` when the read is trustworthy.
    pub hint: Option<&'static str>,
}

/// Category is derived here rather than in the `ocr` crate: `infer_chart_category` owns the keyword
/// table and the `CHART_CATEGORIES` ids, which are persisted *inside the encrypted payload* and
/// grouped on by the charts. A second implementation of those ids would silently orphan expenses.
pub fn prefill_from_scan(fields: &ScanFields) -> ScanPrefill {
    let name = fields.title.as_ref().map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    ScanPrefill {
        category: name.as_deref().map(|n| infer_chart_category(n).to_string()),
        amount: fields.amount,
        date: fields.date.clone(),
        // The one place a low-confidence read is surfaced. A wrong amount that is visible and
        // editable is a nuisance; a wrong amount committed in silence is a wrong ledger.
        hint: (fields.amount.is_none() || !fields.amount_confident).then_some("scan-check-amount"),
        name,
    }
}

#[cfg(test)]
mod tests;
