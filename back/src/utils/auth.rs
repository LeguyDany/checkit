use crate::models::user::User;
use bcrypt::verify;
use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use std::env;
use uuid::Uuid;

use crate::models::{
    auth::{Auth, UserToken},
    response::Response,
};

use rocket::request::{self, FromRequest, Request};
use rocket::{http::Status, outcome::Outcome};

use crate::configs::date::MONTH_IN_SEC;
use crate::models::auth::{AuthorizationError, AuthorizationToken};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationToken {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers = req.headers();

        if let Some(auth_header) = headers.get_one("Authorization") {
            let parts: Vec<&str> = auth_header.split_whitespace().collect();

            if parts.len() == 2 && parts[0] == "Bearer" {
                Outcome::Success(AuthorizationToken(parts[1].to_string()))
            } else {
                Outcome::Failure((Status::Unauthorized, AuthorizationError))
            }
        } else {
            Outcome::Failure((Status::Unauthorized, AuthorizationError))
        }
    }
}

impl Auth {
    pub fn check_pwd_with_userid(user_id: Uuid, pwd: &str) -> Result<bool, Response<String>> {
        let user = User::get_user_by_id(user_id)?.data;
        Ok(verify(pwd, &user.pwd).unwrap())
    }

    pub fn encode_token(user: User) -> Result<Response<String>, Response<String>> {
        let user_info = UserToken {
            userid: user.userid,
            username: user.username,
            isnotionoauth: user.isnotionoauth,
            lastlogin: user.lastlogin,
        };

        let now = (Utc::now().timestamp_nanos() / 1_000_000_000) as u32;

        let payload = Auth {
            user_token: user_info,
            exp: (now + MONTH_IN_SEC) as usize,
        };

        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| Response {
            success: false,
            data: "No secret found.".to_string(),
            status: 400,
        })?;

        let json_encode = jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| Response {
            success: false,
            data: format!("An error has occured: {}", e.to_string()),
            status: 400 as u16,
        })?;

        return Ok(Response {
            success: true,
            data: json_encode.to_string(),
            status: 200,
        });
    }

    pub fn decode_token(token: String) -> Result<Response<Auth>, Response<String>> {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| Response {
            success: false,
            data: "No secret found.".to_string(),
            status: 400,
        })?;

        let json_decode = jsonwebtoken::decode::<Auth>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| Response {
            success: false,
            data: format!("An error has occured: {}", e.to_string()),
            status: 400,
        })?;

        return Ok(Response {
            success: true,
            data: json_decode.claims,
            status: 200,
        });
    }
}
