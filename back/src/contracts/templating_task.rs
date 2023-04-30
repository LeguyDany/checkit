use crate::models::auth::AuthorizationToken;
use crate::models::response::Response;
use crate::models::templating_task::{AddTemplatingTask, TemplatingTask, UpdatedTemplatingTask};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/<templateid>")]
fn get_tasks_from_template(
    templateid: &str,
) -> Result<Json<Response<Vec<TemplatingTask>>>, Json<Response<String>>> {
    use crate::helpers::str_helper::StrChange;

    let template_input_uuid = StrChange::to_uuid(templateid)?;

    let execute =
        TemplatingTask::get_tasks_by_templateid(template_input_uuid).map_err(|e| Json(e))?;

    return Ok(Json(execute));
}

#[post("/add", data = "<data>", format = "application/json")]
fn add_task(
    token: AuthorizationToken,
    data: Json<AddTemplatingTask>,
) -> Result<Json<Response<TemplatingTask>>, Json<Response<String>>> {
    let execute = TemplatingTask::add(&token.0, data.0).map_err(|e| Json(e))?;
    return Ok(Json(execute));
}

#[delete("/delete/<id>")]
fn delete_task(
    id: &str,
    token: AuthorizationToken,
) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let execute = TemplatingTask::delete(&id, &token.0).map_err(|e| Json(e))?;
    return Ok(Json(execute));
}

#[patch("/update", data = "<data>", format = "application/json")]
fn update_task(
    token: AuthorizationToken,
    data: Json<UpdatedTemplatingTask>,
) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let execute = TemplatingTask::update(
        UpdatedTemplatingTask {
            content: data.0.content,
            templateid: data.0.templateid,
            duetime: data.0.duetime,
            temptaskid: data.0.temptaskid,
        },
        &token.0,
    )?;

    return Ok(Json(execute));
}

pub fn routes() -> Vec<Route> {
    routes![delete_task, add_task, update_task, get_tasks_from_template]
}
