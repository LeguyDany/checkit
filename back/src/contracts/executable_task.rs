use crate::models::auth::AuthorizationToken;
use crate::models::executable_task::{AddExeTask, ExeTask, UpdatedExeTask};
use crate::models::response::Response;
use rocket::serde::json::Json;
use rocket::Route;

#[get("/<templateid>")]
fn get_tasks_from_template(
    token: AuthorizationToken,
    templateid: &str,
) -> Result<Json<Response<Vec<ExeTask>>>, Json<Response<String>>> {
    let execute = ExeTask::get_tasks_by_templateid(templateid, &token.0);

    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[post("/add", data = "<data>", format = "application/json")]
fn add_task(
    token: AuthorizationToken,
    data: Json<AddExeTask>,
) -> Result<Json<Response<ExeTask>>, Json<Response<String>>> {
    let execute = ExeTask::add(&token.0, data.0);
    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[get("/add_exetask_by_temptask/<userid>")]
fn add_task_by_temptask(userid: &str) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let execute = ExeTask::add_exetasks_by_temptask(userid);
    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[delete("/delete/<id>")]
fn delete_task(id: &str, token: AuthorizationToken) -> Json<Response<String>> {
    let execute = ExeTask::delete(&id, &token.0);

    match execute {
        Ok(o) => Json(o),
        Err(e) => Json(e),
    }
}

#[patch("/update", data = "<data>", format = "application/json")]
fn update_task(
    token: AuthorizationToken,
    data: Json<UpdatedExeTask>,
) -> Result<Json<Response<String>>, Json<Response<String>>> {
    let execute = ExeTask::update(
        UpdatedExeTask {
            content: data.0.content,
            templateid: data.0.templateid,
            duetime: data.0.duetime,
            checked: data.0.checked,
            exetaskid: data.0.exetaskid,
        },
        &token.0,
    )?;

    return Ok(Json(execute));
}

pub fn routes() -> Vec<Route> {
    routes![
        delete_task,
        add_task,
        update_task,
        get_tasks_from_template,
        add_task_by_temptask
    ]
}
