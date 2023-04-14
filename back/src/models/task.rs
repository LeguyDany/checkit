use crate::schema::schema::task;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedTask {
    pub content: Option<String>,
    pub checked: bool,
    pub templateid: Option<Uuid>,
    pub taskid: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = task)]
pub struct AddTask {
    pub content: Option<String>,
    pub userid: Option<Uuid>,
    pub checked: bool,
    pub templateid: Option<Uuid>,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct Task {
    pub taskid: Uuid,
    pub userid: Option<Uuid>,
    pub templateid: Option<Uuid>,
    pub content: Option<String>,
    pub checked: bool,
    pub creationdate: NaiveDateTime,
    pub duetime: Option<NaiveDateTime>,
}

//
// diesel::table! {
//     task (taskid) {
//         taskid -> Uuid,
//         userid -> Nullable<Uuid>,
//         templateid -> Nullable<Uuid>,
//         content -> Nullable<Varchar>,
//         checked -> Bool,
//         creationdate -> Timestamp,
//         duetime -> Nullable<Timestamp>,
//     }
// }
//
//
