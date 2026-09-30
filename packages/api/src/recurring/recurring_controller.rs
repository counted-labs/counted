use dioxus::{fullstack::Json, prelude::*};
use shared::{
    CreatableRecurringExpense, DeleteRecurringExpenseRequest, EditableRecurringExpense,
    MaterializeRecurringRequest, RecurringExpense,
};

#[post("/api/v1/recurring")]
pub async fn add_recurring_expense(
    Json(payload): Json<CreatableRecurringExpense>,
) -> Result<RecurringExpense, ServerFnError> {
    crate::server::recurring::add_recurring_expense(payload).await
}

#[put("/api/v1/recurring")]
pub async fn edit_recurring_expense(
    Json(payload): Json<EditableRecurringExpense>,
) -> Result<RecurringExpense, ServerFnError> {
    crate::server::recurring::edit_recurring_expense(payload).await
}

#[post("/api/v1/recurring/delete")]
pub async fn delete_recurring_expense(
    Json(req): Json<DeleteRecurringExpenseRequest>,
) -> Result<(), ServerFnError> {
    crate::server::recurring::delete_recurring_expense(req).await
}

#[post("/api/v1/recurring/materialize")]
pub async fn materialize_recurring_expense(
    Json(req): Json<MaterializeRecurringRequest>,
) -> Result<(), ServerFnError> {
    crate::server::recurring::materialize_recurring_expense(req).await
}
