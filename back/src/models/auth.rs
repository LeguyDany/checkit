use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub user_token: UserToken,
    pub exp: usize
}

#[derive(Debug, Serialize,  Deserialize)]
pub struct UserToken {
    pub userid: Uuid,
    pub username: String,
    pub isnotionoauth: bool,
    pub lastlogin: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginId {
    pub username: String,
    pub pwd: String
}

#[derive(Debug, Clone)]
pub struct AuthorizationToken(pub String);

#[derive(Debug)]
pub struct AuthorizationError;