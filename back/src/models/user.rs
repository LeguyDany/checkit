use diesel::prelude::*;

use chrono::NaiveDateTime;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedUser {
    pub id: String,
    pub updated_value: String,
    pub is_username: bool,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub userid: Uuid,
    pub username: String,
    pub pwd: String,
    pub isnotionoauth: bool,
    pub lastlogin: Option<NaiveDateTime>,
    pub datecreated: NaiveDateTime,
}