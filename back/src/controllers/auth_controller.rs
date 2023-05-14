use crate::models::auth::Auth;
use crate::models::response::Response;
use crate::models::user::User;
use bcrypt::verify;
use diesel::prelude::*;

impl Auth {
    pub fn login(
        input_username: &str,
        input_userpwd: &str,
    ) -> Result<Response<String>, Response<String>> {
        let user_from_query: User =
            User::get_user_by_username(input_username).ok_or_else(|| Response {
                success: false,
                data: "User input invalid, the user doesn't exist.".to_string(),
                status: 400,
            })?;

        let password_is_valid =
            verify(input_userpwd, &user_from_query.pwd).map_err(|_| Response {
                success: false,
                data: "Couldn't verify the password. Please, try again.".to_string(),
                status: 500,
            })?;

        if password_is_valid {
            let res = Auth::encode_token(user_from_query.clone()).map_err(|e| Response {
                success: false,
                data: format!("Token encoding failed: {:?}", e),
                status: 500,
            })?;

            use crate::schema::schema::user::dsl::{token, user, userid};
            let conn = &mut back::establish_connection();

            let _delete_user_token = diesel::update(user.filter(userid.eq(user_from_query.userid)))
                .set(token.eq(&res.data))
                .execute(conn);

            return Ok(res);
        }

        return Err(Response {
            data: "Wrong password.".to_string(),
            success: false,
            status: 400,
        });
    }

    pub fn check_login(token: String) -> Result<Response<Auth>, Response<String>> {
        let token_clone = token.clone();
        let decoded = Auth::decode_token(token)?;

        use crate::schema::schema::user::dsl;
        let conn = &mut back::establish_connection();
        if dsl::user
            .filter(dsl::username.eq(&decoded.data.user_token.username))
            .filter(dsl::token.eq(token_clone))
            .get_result::<User>(conn)
            .is_ok()
        {
            return Ok(decoded);
        }
        return Err(Response {
            success: false,
            data: "Your account session is no longer valid. Please, log in again.".to_string(),
            status: 400,
        });
    }

    pub fn logout(encoded_token: String) -> Result<Response<String>, Response<String>> {
        use crate::schema::schema::user::dsl::{token, user, userid};

        let token_decoded = Auth::decode_token(encoded_token.clone())
            .unwrap()
            .data
            .user_token;

        User::get_user_by_id(token_decoded.userid)?.data.token;

        let conn = &mut back::establish_connection();

        let _ = diesel::update(user.filter(userid.eq(&token_decoded.userid)))
            .set(token.eq(None::<String>))
            .execute(conn)
            .map_err(|_| Response {
                success: false,
                data: "Could not set the token to null".to_string(),
                status: 400,
            })?;

        return Ok(Response {
            success: true,
            data: "User successfully logged out.".to_string(),
            status: 200,
        });
    }
}
