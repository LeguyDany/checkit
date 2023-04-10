use diesel::prelude::*;

use chrono::NaiveDateTime;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedUser {
    pub is_username: bool,
    pub updated_value: String,
    pub pwd: String,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct User {
    pub userid: Uuid,
    pub username: String,
    pub pwd: String,
    pub isnotionoauth: bool,
    pub lastlogin: Option<NaiveDateTime>,
    pub datecreated: NaiveDateTime,
    pub token: Option<String>,
}