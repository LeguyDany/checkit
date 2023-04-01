use diesel::{Insertable};
use crate::schema::user;

#[path ="../models/user.rs"] pub mod user_model;
use self::user_model::User;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct AddUser<'a> {
    pub username: &'a str,
    pub pwd: &'a str,
}

impl User {

    pub fn add(conn: &mut PgConnection, username: &str, pwd: &str) -> User {
    
        let new_post = AddUser { username, pwd };
    
        diesel::insert_into(user::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Error saving new post")
    }

    pub fn read() {
        use crate::schema::user::dsl::*;
    
        let connection = &mut crate::establish_connection();
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
