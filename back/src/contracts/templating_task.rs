use crate::models::auth::AuthorizationToken;
use crate::models::response::Response;
use crate::models::templating_task::{AddTemplatingTask, TemplatingTask, UpdatedTemplatingTask};
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[get("/<templateid>")]
fn get_tasks_from_template(
    token: AuthorizationToken,
    templateid: &str,
) -> Result<Json<Response<Vec<TemplatingTask>>>, Json<Response<String>>> {
    use crate::helpers::str_helper::StrChange;

    let template_input_uuid: Uuid;

    match StrChange::to_uuid(templateid) {
        Ok(o) => {
            template_input_uuid = o;
        }
        Err(e) => {
            return Err(Json(Response {
                success: false,
                data: e.to_string(),
                status: 400,
            }));
        }
    }
    let execute = TemplatingTask::get_tasks_by_templateid(template_input_uuid, &token.0);

    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[post("/add", data = "<data>", format = "application/json")]
fn add_task(
    token: AuthorizationToken,
    data: Json<AddTemplatingTask>,
) -> Result<Json<Response<TemplatingTask>>, Json<Response<String>>> {
    let execute = TemplatingTask::add(&token.0, data.0);
    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[delete("/delete/<id>")]
fn delete_task(id: &str, token: AuthorizationToken) -> Json<Response<String>> {
    println!("bonjour");
    let execute = TemplatingTask::delete(&id, &token.0);

    match execute {
        Ok(o) => Json(o),
        Err(e) => Json(e),
    }
}

#[patch("/update", data = "<data>", format = "application/json")]
fn update_task(
    token: AuthorizationToken,
    data: Json<UpdatedTemplatingTask>,
) -> Json<Response<String>> {
    let execute = TemplatingTask::update(
        UpdatedTemplatingTask {
            content: data.0.content,
            templateid: data.0.templateid,
            duetime: data.0.duetime,
            temptaskid: data.0.temptaskid,
        },
        &token.0,
    );

    Json(execute)
}

pub fn routes() -> Vec<Route> {
    routes![delete_task, add_task, update_task, get_tasks_from_template]
}
