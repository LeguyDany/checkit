use crate::schema::schema::executable_task;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedExeTask {
    pub content: Option<String>,
    pub duetime: Option<NaiveDateTime>,
    pub checked: bool,
    pub templateid: Option<Uuid>,
    pub exetaskid: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = executable_task)]
pub struct AddExeTask {
    pub content: Option<String>,
    pub userid: Option<Uuid>,
    pub duetime: Option<NaiveDateTime>,
    pub checked: bool,
    pub templateid: Option<Uuid>,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct ExeTask {
    pub exetaskid: Uuid,
    pub userid: Option<Uuid>,
    pub templateid: Option<Uuid>,
    pub content: Option<String>,
    pub checked: bool,
    pub creationdate: NaiveDateTime,
    pub duetime: Option<NaiveDateTime>,
}

//
// diesel::table! {
//     executable_task (exetaskid) {
//         exetaskid -> Uuid,
//         userid -> Nullable<Uuid>,
//         templateid -> Nullable<Uuid>,
//         content -> Nullable<Varchar>,
//         checked -> Bool,
//         creationdate -> Timestamp,
//         duetime -> Nullable<Timestamp>,
//     }
// }
