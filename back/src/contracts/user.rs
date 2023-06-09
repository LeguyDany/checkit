use crate::models::auth::AuthorizationToken;
use crate::models::user::{UpdatedUser, User};
use crate::{controllers::user_controller::AddUser, models::response::Response};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/<name>")]
fn search_users(name: &str) -> Result<Json<Response<User>>, Json<Response<String>>> {
    let users = User::get_user_by_username(name).ok_or_else(|| Response {
        success: false,
        data: "Could not find any user by that username.".to_string(),
        status: 200,
    })?;

    return Ok(Json(Response {
        success: true,
        data: users,
        status: 200,
    }));
}

#[get("/getCurrentUser")]
fn get_current_user(
    token: AuthorizationToken,
) -> Result<Json<Response<User>>, Json<Response<String>>> {
    let execute = User::get_current_user(token.0);
    Ok(Json(execute?))
}

#[get("/getUsers/<num>")]
fn get_users(num: &str) -> Json<Response<Vec<User>>> {
    let users = User::read(num.parse().unwrap());
    return Json(users);
}

#[get("/getAllUsers")]
fn get_all_users() -> Json<Response<Vec<User>>> {
    let users = User::get_all_users();
    return Json(users);
}

#[post("/addUser", data = "<user>", format = "application/json")]
fn add_user(user: Json<AddUser>) -> Result<Json<Response<User>>, Json<Response<String>>> {
    let users = User::add(user.username, user.pwd);
    match users {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[delete("/delete/<id>")]
fn delete_user(id: &str) -> Json<Response<String>> {
    let execute = User::delete(&id);

    match execute {
        Ok(o) => Json(o),
        Err(e) => Json(e),
    }
}

#[patch("/update_user", data = "<data>", format = "application/json")]
fn update_user(
    token: AuthorizationToken,
    data: Json<UpdatedUser>,
) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let execute = User::update(token.0, &data.is_username, &data.updated_value, &data.pwd);
    Ok(Json(execute?))
}

pub fn routes() -> Vec<Route> {
    routes![
        search_users,
        get_users,
        add_user,
        delete_user,
        update_user,
        get_all_users,
        get_current_user
    ]
}
