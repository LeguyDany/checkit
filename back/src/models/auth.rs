use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    token: String,
    exp: NaiveDateTime,
    creation_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginId {
    pub id: String,
}