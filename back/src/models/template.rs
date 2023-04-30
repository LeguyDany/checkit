use crate::schema::schema::template;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedTemplate {
    pub template_name: String,
    pub weekdays: Weekdays,
    pub templateid: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = template)]
pub struct AddTemplate<'a> {
    pub templatename: &'a str,
    pub weekdays: Vec<bool>,
    pub userid: Option<Uuid>,
}

#[derive(Debug, Deserialize, serde::Serialize, Clone)]
pub struct Weekdays {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Queryable, serde::Serialize, Clone)]
pub struct Template {
    pub templateid: Uuid,
    pub userid: Option<Uuid>,
    pub templatename: String,
    pub creationdate: NaiveDateTime,
    pub weekdays: Vec<Option<bool>>,
    pub updatedate: Option<NaiveDateTime>,
}
