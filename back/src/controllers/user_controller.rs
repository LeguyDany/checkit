use crate::{models::response::Response, schema::schema::user};
use diesel::Insertable;
use serde::Deserialize;
use uuid::Uuid;

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
    pub fn get_user_by_id(id: Uuid) -> User {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::user;

        let mut user_filtered = user
            .find(id)
            .load::<User>(conn)
            .expect("Error loading posts");

        let response: User = user_filtered.remove(0);
        response
    }

    pub fn get_user_by_username(inputname: &str) -> Option<User> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::user::dsl::{user, username};

        user
            .filter(username.eq(inputname))
            .first::<User>(conn)
            .ok()
    }

    pub fn delete(id: &str) -> Vec<String> {
        use crate::helpers::str_helper::StrChange;

        let mut response = Vec::new();
        let input_uuid: Uuid;

        match StrChange::to_uuid(id) {
            Ok(o) => {
                input_uuid = o;
            }
            Err(x) => {
                response.push("Error".to_owned());
                response.push(x);
                return response;
            }
        }

        let conn = &mut back::establish_connection();

        use crate::schema::schema::user::dsl::{user, userid};
        match diesel::delete(user.filter(userid.eq(input_uuid))).execute(conn) {
            Ok(res) => {
                if res.to_string().parse::<i32>().unwrap() > 0 {
                    response.push("Success".to_owned());
                    response.push(format!("{} user deleted.", res.to_string()));
                } else {
                    response.push("Error".to_owned());
                    response.push("There isn't any user with that uuid.".to_owned());
                }
            }
            Err(e) => {
                response.push("Error".to_owned());
                response.push(e.to_string());
            }
        }

        response
    }

    pub fn update(id: &str, update_value: &str, is_username: &bool) -> Result<User, String> {
        use crate::helpers::str_helper::StrChange;
        use crate::schema::schema::user::dsl::{pwd, user, userid, username};
        let user_uuid = StrChange::to_uuid(id).unwrap();

        let conn = &mut back::establish_connection();

        let updated_user;

        if *is_username {
            updated_user = diesel::update(user.filter(userid.eq(user_uuid)))
                .set(username.eq(update_value))
                .get_result::<User>(conn);
        } else {
            let password_hash = hash(update_value.trim_end(), DEFAULT_COST).unwrap();
            updated_user = diesel::update(user.filter(userid.eq(user_uuid)))
                .set(pwd.eq(password_hash))
                .get_result::<User>(conn);
        }

        match updated_user {
            Ok(o) => return Ok(o),
            Err(e) => return Err(e.to_string()),
        }
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
        let conn = &mut back::establish_connection();

        let already_exist:Option<User> = User::get_user_by_username(username_input);
        if already_exist.is_some() {
            return Err(Response {
                success: false,
                data: "User already exists. Pick another username.".to_string(),
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
        })
    }

    pub fn read(num_user: i64) -> Vec<User> {
        use crate::schema::schema::user::dsl::*;

        let connection = &mut back::establish_connection();
        let results = user
            .filter(isnotionoauth.eq(false))
            .limit(num_user)
            .load::<User>(connection)
            .expect("Error loading posts");

        return results;
    }
}