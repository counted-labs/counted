use crate::User;
use crate::time::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UsersProject {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<User>,
    pub created_at: DateTime,
    pub currency: String,
    pub description: Option<String>,
}
