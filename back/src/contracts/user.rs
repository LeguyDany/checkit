use std::collections::HashMap;

use crate::{controllers::user_controller::AddUser, models::response::Response};
use rocket::serde::json::Json;
use rocket::Route;

use crate::models::user::{User, UpdatedUser};

#[get("/<name>")]
fn search_users(name: &str) -> Result<Json<Response<User>>, Json<Response<String>>> {
    let users = User::get_user_by_username(name);
    match users {
        Some(o) => Ok(Json(Response{success: true, data:o})),
        None => Err(Json(Response{success: false, data:"Could not find any user by that username.".to_string()}))
    }
    
}

#[get("/getUsers/<num>")]
fn get_users(num: &str) -> Json<Vec<User>> {
    let users = User::read(num.parse().unwrap());
    return Json(users);
}

#[post("/addUser", data = "<user>", format = "application/json")]
fn add_user(user: Json<AddUser>) -> Result<Json<Response<User>>, Json<Response<String>>> {
    let users = User::add(user.username, user.pwd);
    match users {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e))
    }
}

#[delete("/delete/<id>")]
fn delete_user(id: &str) -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();

    let execute = User::delete(&id);
    response.insert(execute[0].to_string(), execute[1].to_string());

    Json(response)
}

#[patch("/update_user", data = "<data>", format = "application/json")]
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
