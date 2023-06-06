use crate::models::auth::AuthorizationError;
use crate::models::fairings::HeaderValidator;
use dotenvy::dotenv;
use rocket::request::{self, FromRequest, Request};
use rocket::{http::Status, outcome::Outcome};
use std::env;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeaderValidator {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers = req.headers();

        if let Some(api_key) = headers.get_one("API_KEY") {
            dotenv().ok();
            let env_api_key = env::var("API_KEY").expect("API_KEY must be set");

            if api_key == env_api_key {
                Outcome::Success(HeaderValidator {})
            } else {
                Outcome::Failure((Status::Unauthorized, AuthorizationError))
            }
        } else {
            Outcome::Failure((Status::Unauthorized, AuthorizationError))
        }
    }
}
