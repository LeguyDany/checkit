use crate::models::auth::AuthorizationToken;
use crate::models::auth::{Auth, LoginId};
use crate::models::response::Response;
use rocket::serde::json::Json;
use rocket::Route;

#[post("/login", data = "<data>", format = "application/json")]
fn login(data: Json<LoginId>) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let res = Auth::login(&data.username, &data.pwd);
    return Ok(Json(res?));
}

#[get("/check_logged_in")]
fn check_user_is_logged_in(
    token: AuthorizationToken,
) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let res = Auth::check_login(token.0).map_err(|e| Json(e))?;

    return Ok(Json(Response {
        success: true,
        data: format!("Success: {}", res.data.user_token.username),
        status: res.status,
    }));
}

#[get("/logout")]
fn logout(token: AuthorizationToken) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let res = Auth::logout(token.0).map_err(|e| Json(e))?;
    Ok(Json(res))
}

pub fn routes() -> Vec<Route> {
    routes![login, check_user_is_logged_in, logout]
}
