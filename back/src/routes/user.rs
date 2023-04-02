use rocket::serde::json::Json;
use rocket::{get, Route};
use crate::controllers::userController::user_model::User;

#[get("/<name>")]
fn search_users(name: &str) -> Json<Vec<User>> {
    let users = User::get_id_by_username(name);
    return Json(users);
}

#[get("/test/<name>")]
fn search_users2(name: &str) -> Json<Vec<User>> {
    let users = User::get_id_by_username(name);
    return Json(users);
}

#[get("/test2/<name>")]
fn search_users3(name: &str) -> Json<Vec<User>> {
    let users = User::get_id_by_username(name);
    return Json(users);
}

pub fn routes() -> Vec<Route> {
    routes![search_users, search_users2, search_users3]
}