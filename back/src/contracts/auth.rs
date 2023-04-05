use rocket::serde::json::Json;
use rocket::Route;
use crate::models::auth::{Auth, LoginId};
use crate::models::response::{Response};
use crate::models::auth::{AuthorizationToken};

#[post("/login", data = "<data>", format = "application/json")]
fn login(data: Json<LoginId>) -> Json<Response<String>> {
    let res = Auth::login(&data.username, &data.pwd);
    return Json(res);
}

#[get("/check_logged_in")]
fn check_user_is_logged_in(token: AuthorizationToken) -> Json<Response<String>> {
    let res = Auth::check_login(token.0);

    match res {
        Ok(o) => {
            return Json(Response{success: true, data: format!("Success: {}", o.data.user_token.username), status: o.status});
        },
        Err(e) => {
            return Json(Response{success: false, data: format!("Error: {}", e.data), status: e.status});
        }
    }
}

#[get("/logout")]
fn logout(token: AuthorizationToken) -> Json<Response<String>> {
    let res = Auth::logout(token.0);
    Json(res)
}

pub fn routes() -> Vec<Route> {
    routes![
        login,
        check_user_is_logged_in,
        logout
    ]
}
