#[macro_use] extern crate rocket;
#[path = "./routes/user.rs"] pub mod user_route;

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
