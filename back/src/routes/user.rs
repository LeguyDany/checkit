use rocket::serde::json::Json;
use rocket::{get, Route};
use crate::controllers::userController::user_model::User;

#[get("/<name>")]
fn search_users(name: &str) -> Json<Vec<User>> {
    let users = User::get_user_by_username(name);
    return Json(users);
}

#[get("/getUsers/<num>")]
fn get_users(num: &str) -> Json<Vec<User>> {
    let users = User::read(num.parse().unwrap());
    return Json(users);
}

#[get("/test2/<name>")]
fn search_users3(name: &str) -> Json<Vec<User>> {
    let users = User::get_user_by_username(name);
    return Json(users);
}

pub fn routes() -> Vec<Route> {
    routes![search_users, get_users, search_users3]
}