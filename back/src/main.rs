#[macro_use] extern crate rocket;

mod controllers;
mod routes;

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
