#[macro_use] extern crate rocket;

mod helpers;
mod controllers;
mod contracts;
mod models;
mod utils;
mod schema;
mod configs;

use self::contracts::user;
use self::contracts::auth;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/users", user::routes())
        .mount("/auth", auth::routes())
}
