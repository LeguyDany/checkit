use crate::schema::schema::templating_task;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedTemplatingTask {
    pub content: Option<String>,
    pub duetime: Option<NaiveDateTime>,
    pub templateid: Option<Uuid>,
    pub temptaskid: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = templating_task)]
pub struct AddTemplatingTask {
    pub content: Option<String>,
    pub userid: Option<Uuid>,
    pub duetime: Option<NaiveDateTime>,
    pub templateid: Option<Uuid>,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct TemplatingTask {
    pub temptaskid: Uuid,
    pub userid: Option<Uuid>,
    pub templateid: Option<Uuid>,
    pub content: Option<String>,
    pub creationdate: NaiveDateTime,
    pub duetime: Option<NaiveDateTime>,
}

//
// diesel::table! {
//     templating_task (temptaskid) {
//         temptaskid -> Uuid,
//         userid -> Nullable<Uuid>,
//         templateid -> Nullable<Uuid>,
//         content -> Nullable<Varchar>,
//         creationdate -> Timestamp,
//         duetime -> Nullable<Timestamp>,
//     }
// }
//
