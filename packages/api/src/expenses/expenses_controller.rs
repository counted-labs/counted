use dioxus::{fullstack::Json, prelude::*};
use shared::{
    CreatableExpense, DeleteExpenseRequest, EditableExpense, Expense, ExpenseWithPayments,
};
use uuid::Uuid;

#[post("/api/v1/expenses")]
pub async fn add_expense(
    Json(expense): Json<CreatableExpense>,
) -> Result<ExpenseWithPayments, ServerFnError> {
    crate::server::expenses::add_expense(expense).await
}

#[post("/api/v1/expenses/batch")]
pub async fn batch_add_expenses(
    Json(expenses): Json<Vec<CreatableExpense>>,
) -> Result<(), ServerFnError> {
    crate::server::expenses::batch_add_expenses(expenses).await
}

#[put("/api/v1/expenses")]
pub async fn edit_expense(
    Json(expense): Json<EditableExpense>,
) -> Result<ExpenseWithPayments, ServerFnError> {
    crate::server::expenses::edit_expense(expense).await
}

#[get("/api/v1/projects/{project_id}/expenses")]
pub async fn get_expenses_by_project_id(project_id: Uuid) -> Result<Vec<Expense>, ServerFnError> {
    crate::server::expenses::get_expenses_by_project_id(project_id).await
}

#[get("/api/v1/projects/{project_id}/expenses/{expense_id}")]
pub async fn get_expense_by_id(
    project_id: Uuid,
    expense_id: i32,
) -> Result<Expense, ServerFnError> {
    crate::server::expenses::get_expense_by_id(project_id, expense_id).await
}

#[post("/api/v1/expenses/delete")]
pub async fn delete_expense(Json(req): Json<DeleteExpenseRequest>) -> Result<(), ServerFnError> {
    crate::server::expenses::delete_expense(req).await
}
