use rocket::serde::json::Json;
use rocket::Route;

use crate::models::auth::{Auth, LoginId};
use crate::models::response::{Response};


#[post("/login", data = "<data>", format = "application/json")]
fn login(data: Json<LoginId>) -> Json<Response<String>> {
    let res = Auth::login(&data.id);
    return Json(res);
}


pub fn routes() -> Vec<Route> {
    routes![
        login,
    ]
}
