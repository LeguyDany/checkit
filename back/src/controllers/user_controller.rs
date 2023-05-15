use crate::{models::response::Response, schema::schema::user};
use diesel::Insertable;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::auth::Auth;
use crate::models::user::User;
use diesel::prelude::*;

use bcrypt::{hash, DEFAULT_COST};

/// # Arguments
///
/// pub username: &'a str,
/// pub pwd: &'a str,
#[derive(Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct AddUser<'a> {
    pub username: &'a str,
    pub pwd: &'a str,
}

impl User {
    pub fn get_user_by_id(id: Uuid) -> Result<Response<User>, Response<String>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::user;

        let mut user_filtered = user
            .find(id)
            .load::<User>(conn)
            .expect("Error loading posts");

        let response: User = user_filtered.remove(0);
        Ok(Response {
            success: true,
            data: response,
            status: 200,
        })
    }

    pub fn get_current_user(token: String) -> Result<Response<User>, Response<String>> {
        let decoded_token = Auth::decode_token(token)?.data;

        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::{user, userid};

        let res = user
            .filter(userid.eq(decoded_token.user_token.userid))
            .first::<User>(conn)
            .ok()
            .ok_or_else(|| Response {
                success: false,
                data: "User does not exist or an issue occured while looking for the user."
                    .to_string(),
                status: 404,
            })?;

        return Ok(Response {
            success: true,
            data: res,
            status: 200,
        });
    }

    pub fn get_user_by_username(inputname: &str) -> Option<User> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::{user, username};

        user.filter(username.eq(inputname)).first::<User>(conn).ok()
    }

    pub fn get_all_users() -> Response<Vec<User>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::user;

        let all_users = user.load::<User>(conn).unwrap();
        Response {
            success: true,
            data: all_users,
            status: 200,
        }
    }

    pub fn delete(id: &str) -> Result<Response<String>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let input_uuid = StrChange::to_uuid(id)?;

        let conn = &mut back::establish_connection();

        use crate::schema::schema::user::dsl::{user, userid};

        let res = diesel::delete(user.filter(userid.eq(input_uuid)))
            .execute(conn)
            .map_err(|e| Response {
                success: false,
                data: e.to_string(),
                status: 400,
            })?;

        if res.to_string().parse::<i32>().unwrap() > 0 {
            Ok(Response {
                success: true,
                data: format!("{} user deleted.", res.to_string()),
                status: 200,
            })
        } else {
            Ok(Response {
                success: false,
                data: "There isn't any user with that uuid.".to_owned(),
                status: 404,
            })
        }
    }

    pub fn update(
        token: String,
        is_username: &bool,
        updated_value: &str,
        user_pwd: &str,
    ) -> Result<Response<String>, Response<String>> {
        if updated_value.len() < 4 {
            return Err(Response {
                success: false,
                data: "Please, enter new value with more than 4 characters.".to_string(),
                status: 400,
            });
        };
        if is_username == &true && updated_value.len() > 20 {
            return Err(Response {
                success: false,
                data: "Please, enter an username with less than 20 characters.".to_string(),
                status: 400,
            });
        };
        if is_username == &true && User::get_user_by_username(updated_value).is_some() {
            return Err(Response {
                success: false,
                data: "This username already exists, please choose another one.".to_string(),
                status: 500,
            });
        }

        use crate::schema::schema::user::dsl::{pwd, user, userid, username};

        let decoded_token = Auth::decode_token(token)?.data;

        if !Auth::check_pwd_with_userid(decoded_token.user_token.userid, user_pwd)? {
            return Ok(Response {
                success: false,
                data: "Wrong password.".to_string(),
                status: 400,
            });
        }

        let conn = &mut back::establish_connection();

        let updated_user;

        if *is_username {
            let _data = diesel::update(user.filter(userid.eq(decoded_token.user_token.userid)))
                .set(username.eq(updated_value))
                .execute(conn);
            updated_user = Auth::login(updated_value, user_pwd)?
        } else {
            let password_hash =
                hash(updated_value.trim_end(), DEFAULT_COST).map_err(|e| Response {
                    success: false,
                    data: format!("Error: {}", e.to_string()),
                    status: 400,
                })?;
            let _data = diesel::update(user.filter(userid.eq(decoded_token.user_token.userid)))
                .set(pwd.eq(password_hash))
                .execute(conn);
            updated_user = Auth::login(&decoded_token.user_token.username, updated_value)?
        }

        return Ok(updated_user);
    }

    /// Adds a new user to the database.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the new user.
    /// * `pwd` - The password of the new user.
    ///
    /// # Example
    ///
    /// ```
    /// let mut username = String::new();
    /// let mut pwd = String::new();
    ///
    ///  println!("Account creation. Enter a username.");
    ///  stdin().read_line(&mut username).unwrap();
    ///
    ///  println!("Ok! What's your password {}?", username);
    ///  stdin().read_line(&mut pwd).unwrap();
    ///
    ///  let post = User::add(&username, &pwd);
    /// ```
    pub fn add(username_input: &str, pwd: &str) -> Result<Response<User>, Response<String>> {
        if username_input.len() < 4 {
            return Err(Response {
                success: false,
                data: "Username too short, please enter at least 4 characters".to_string(),
                status: 400,
            });
        }
        if username_input.len() > 20 {
            return Err(Response {
                success: false,
                data: "Username too long, please enter at most 20 characters".to_string(),
                status: 400,
            });
        }
        if pwd.len() < 4 {
            return Err(Response {
                success: false,
                data: "Password too short, please enter at least 4 characters".to_string(),
                status: 400,
            });
        }

        let conn = &mut back::establish_connection();

        let already_exist: Option<User> = User::get_user_by_username(username_input);
        if already_exist.is_some() {
            return Err(Response {
                success: false,
                data: "User already exists. Pick another username.".to_string(),
                status: 200,
            });
        }

        let password_hash = hash(pwd.trim_end(), DEFAULT_COST).unwrap();
        let new_post = AddUser {
            username: username_input.trim_end(),
            pwd: &password_hash,
        };

        Ok(Response {
            success: true,
            data: diesel::insert_into(user::table)
                .values(&new_post)
                .get_result(conn)
                .expect("Error saving new post: {}"),
            status: 200,
        })
    }

    pub fn read(num_user: i64) -> Response<Vec<User>> {
        use crate::schema::schema::user::dsl::*;

        let connection = &mut back::establish_connection();
        let results = user
            .filter(isnotionoauth.eq(false))
            .limit(num_user)
            .load::<User>(connection)
            .expect("Error loading posts");

        return Response {
            data: results,
            success: true,
            status: 200,
        };
    }
}
