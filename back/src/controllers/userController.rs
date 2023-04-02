use back::schema::user;
use diesel::Insertable;

#[path = "../models/user.rs"]
pub mod user_model;
use self::user_model::User;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct AddUser<'a> {
    pub username: &'a str,
    pub pwd: &'a str,
}

impl User {
    pub fn get_id_by_username(inputname: &str) -> Vec<User> {
        let conn = &mut back::establish_connection();
        use back::schema::user::dsl::{user, username};

        let users = user
            .filter(username.eq(inputname.trim()))
            .load::<User>(conn)
            .expect("Error loading posts");

        return users;
    }

    pub fn add(username: &str, pwd: &str) -> User {
        let conn = &mut back::establish_connection();
        let new_post = AddUser {
            username: username.trim_end(),
            pwd: pwd.trim_end(),
        };

        diesel::insert_into(user::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Error saving new post")
    }

    pub fn read() {
        use back::schema::user::dsl::*;

        let connection = &mut back::establish_connection();
        let results = user
            .filter(isnotionoauth.eq(false))
            .limit(5)
            .load::<User>(connection)
            .expect("Error loading posts");

        println!("Displaying {} users:", results.len());
        for guy in results {
            println!("Username: {} | Password: {}", guy.username, guy.pwd);
        }
    }
}