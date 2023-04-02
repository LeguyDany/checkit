use diesel::sql_types::Json;
// use chrono::NaiveDateTime;
// use jsonwebtoken::errors::Result;
// use jsonwebtoken::TokenData;
// use jsonwebtoken::{DecodingKey, EncodingKey};
// use jsonwebtoken::{Header, Validation};
// use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers::str_helper::StrChange;
use crate::models::auth::Auth;
use crate::models::response::Response;
use crate::models::user::User;


// Source : https://github.com/SakaDream/rocket-rest-api-with-jwt/blob/master/src/jwt.rs

impl Auth {
    pub fn login(userid: &str) -> Response<String> {
        let user_uuid: Uuid;
        match StrChange::to_uuid(userid) {
            Ok(o) => user_uuid = o,
            Err(e) => {
                return Response {
                    success: false,
                    data: format!("An error has occured: {}", e.to_string()),
                }
            }
        }

        let user: User = User::get_user_by_id(user_uuid);

        // payload
        // EncodingKey
        // Save token in database
        // return token

        Response {
            success: true,
            data: format!("User is now logged in."),
        }
    }
}
