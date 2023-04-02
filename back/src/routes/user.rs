use std::collections::HashMap;

use crate::controllers::user_controller::AddUser;
use rocket::serde::json::Json;
use rocket::Route;

use crate::models::user::{User, UpdatedUser};

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

#[post("/addUser", data = "<user>", format = "application/json")]
fn add_user(user: Json<AddUser>) -> Json<User> {
    let users = User::add(user.username, user.pwd);
    return Json(users);
}

#[delete("/delete/<id>")]
fn delete_user(id: &str) -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();

    let execute = User::delete(&id);
    response.insert(execute[0].to_string(), execute[1].to_string());

    Json(response)
}


#[put("/update_user", data = "<data>", format = "application/json")]
fn update_user(data: Json<UpdatedUser>) -> Json<Result<User, String>> {
    let execute = User::update(&data.id, &data.updated_value, &data.is_username);

    match execute {
        Ok(o) => Json(Ok(o)),
        Err(e) => Json(Err(format!(
            "Oops, something didn't go well!\nError: {}",
            e.to_string()
        ))),
    }
}

pub fn routes() -> Vec<Route> {
    routes![
        search_users,
        get_users,
        add_user,
        delete_user,
        update_user
    ]
}
