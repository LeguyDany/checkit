use std::env;

use rocket::futures::stream::PollNext;
// use diesel::sql_types::Json;
// use jsonwebtoken::errors::Result;
// use jsonwebtoken::TokenData;
use uuid::Uuid;

use diesel::prelude::*;

use crate::helpers::str_helper::StrChange;
use crate::models::auth::Auth;
use crate::models::response::Response;
use crate::models::user::User;
use bcrypt::verify;

// Source : https://github.com/SakaDream/rocket-rest-api-with-jwt/blob/master/src/jwt.rs
impl Auth {
    pub fn login(input_username: &str, input_userpwd: &str) -> Response<String> {
        let user_from_input: Vec<User> = User::get_user_by_username(input_username);
        for element in user_from_input.iter() {
            let account_valid = verify(input_userpwd, &element.pwd).unwrap();
            if account_valid {
                let res = Auth::encode_token((*element).clone());

                match res {
                    Ok(o) => {
                        {
                            use crate::schema::schema::user::dsl::{token, user, userid};
                            let conn = &mut back::establish_connection();

                            let _test = diesel::update(user.filter(userid.eq(element.userid)))
                                .set(token.eq(&o.data))
                                .execute(conn);
                        }
                        return o;
                    }
                    Err(e) => return e,
                }
            }
            return Response {
                data: "Wrong password.".to_string(),
                success: false,
            };
        }

        Response {
            data: "No user with this username.".to_string(),
            success: false,
        }
    }

    pub fn check_login(token: String) -> Result<Response<Auth>, Response<String>> {
        let token_clone = token.clone();
        let decoded = Auth::decode_token(token);

        match decoded {
            Ok(o) => {
                use crate::schema::schema::user::dsl;
                let conn = &mut back::establish_connection();
                if dsl::user
                    .filter(dsl::username.eq(&o.data.user_token.username))
                    .filter(dsl::token.eq(token_clone))
                    .get_result::<User>(conn)
                    .is_ok()
                {
                    return Ok(o);
                }
                return Err(Response{success: false, data: "You are trying to login with a non-valid token. Please, log in again.".to_string()})
            }
            Err(e) => Err(e),
        }
    }
}
