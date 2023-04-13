use crate::models::auth::AuthorizationToken;
use crate::models::response::Response;
use crate::models::template::{AddTemplate, Template, UpdatedTemplate};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/")]
fn get_account_templates(
    token: AuthorizationToken,
) -> Result<Json<Response<Vec<Template>>>, Json<Response<String>>> {
    let execute = Template::get_template_by_userid(&token.0);

    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[post("/add", data = "<data>", format = "application/json")]
fn add_template(
    token: AuthorizationToken,
    data: Json<AddTemplate>,
) -> Result<Json<Response<Template>>, Json<Response<String>>> {
    let execute = Template::add(&token.0, data.0);
    match execute {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(Json(e)),
    }
}

#[delete("/delete/<id>")]
fn delete_template(id: &str, token: AuthorizationToken) -> Json<Response<String>> {
    let execute = Template::delete(&id, &token.0);

    match execute {
        Ok(o) => Json(o),
        Err(e) => Json(e),
    }
}

#[patch("/update", data = "<data>", format = "application/json")]
fn update_template(
    token: AuthorizationToken,
    data: Json<UpdatedTemplate>,
) -> Json<Response<String>> {
    let execute = Template::update(
        UpdatedTemplate {
            template_name: data.0.template_name,
            templateid: data.0.templateid,
            weekdays: data.0.weekdays,
        },
        &token.0,
    );

    Json(execute)
}

pub fn routes() -> Vec<Route> {
    routes![
        delete_template,
        add_template,
        get_account_templates,
        update_template
    ]
}
