use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub account: String,
    pub password: String,
    pub name: String,
    pub sex: String,
    pub phone: String,
    pub email: String,
}
