#[macro_use] extern crate rocket;

mod helpers;
mod controllers;
mod routes;
mod models;

use self::routes::user::routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/users", routes())
}
