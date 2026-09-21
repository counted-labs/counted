use crate::User;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UsersProject {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<User>,
    pub created_at: NaiveDateTime,
    pub currency: String,
    pub description: Option<String>,
}
