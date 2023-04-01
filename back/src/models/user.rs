use diesel::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub userid: Uuid,
    pub username: String,
    pub pwd: String,
    pub isnotionoauth: bool,
    pub lastlogin: Option<NaiveDateTime>,
    pub datecreated: NaiveDateTime,
}