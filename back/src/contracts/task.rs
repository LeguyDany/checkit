use crate::models::auth::AuthorizationToken;
use crate::models::response::Response;
use crate::models::task::{AddTask, Task, UpdatedTask};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/<templateid>")]
fn get_tasks_from_template(
    token: AuthorizationToken,
    templateid: &str,
) -> Result<Json<Response<Vec<Task>>>, Json<Response<String>>> {
    let execute = Task::get_tasks_by_templateid(templateid, &token.0);

    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[post("/add", data = "<data>", format = "application/json")]
fn add_task(
    token: AuthorizationToken,
    data: Json<AddTask>,
) -> Result<Json<Response<Task>>, Json<Response<String>>> {
    let execute = Task::add(&token.0, data.0);
    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[delete("/delete/<id>")]
fn delete_task(id: &str, token: AuthorizationToken) -> Json<Response<String>> {
    println!("bonjour");
    let execute = Task::delete(&id, &token.0);

    match execute {
        Ok(o) => Json(o),
        Err(e) => Json(e),
    }
}

#[patch("/update", data = "<data>", format = "application/json")]
fn update_task(token: AuthorizationToken, data: Json<UpdatedTask>) -> Json<Response<String>> {
    let execute = Task::update(
        UpdatedTask {
            content: data.0.content,
            templateid: data.0.templateid,
            checked: data.0.checked,
            taskid: data.0.taskid,
        },
        &token.0,
    );

    Json(execute)
}

pub fn routes() -> Vec<Route> {
    routes![delete_task, add_task, update_task, get_tasks_from_template]
}
