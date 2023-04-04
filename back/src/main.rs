#[macro_use] extern crate rocket;

mod helpers;
mod controllers;
mod routes;
mod models;
mod utils;
mod schema;
mod configs;

use self::routes::user;
use self::routes::auth;

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
