#[macro_use]
extern crate rocket;

mod configs;
mod contracts;
mod controllers;
mod helpers;
mod models;
mod schema;
mod utils;

use self::contracts::auth;
use self::contracts::task;
use self::contracts::template;
use self::contracts::user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/users", user::routes())
        .mount("/template", template::routes())
        .mount("/auth", auth::routes())
        .mount("/task", task::routes())
}
