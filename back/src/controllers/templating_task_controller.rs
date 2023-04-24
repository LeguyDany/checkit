use crate::models::auth::Auth;
use crate::models::response::Response;
use uuid::Uuid;

use crate::models::templating_task::{AddTemplatingTask, TemplatingTask, UpdatedTemplatingTask};
use diesel::prelude::*;

use rocket::http::Status;
use rocket::response::status;

impl TemplatingTask {
    pub fn get_task_by_id(id: &str) -> Result<Response<TemplatingTask>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let input_uuid: Uuid;

        match StrChange::to_uuid(id) {
            Ok(o) => {
                input_uuid = o;
            }
            Err(e) => {
                println!("{}", status::Custom(Status::BadRequest, e.to_string()).0);
                return Err(Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                });
            }
        }

        let conn = &mut back::establish_connection();
        use crate::schema::schema::templating_task::dsl::templating_task;

        let mut task_filtered = templating_task
            .find(input_uuid)
            .load::<TemplatingTask>(conn)
            .expect("Err loading tasks");

        let response: TemplatingTask = task_filtered.remove(0);
        Ok(Response {
            success: true,
            data: response,
            status: 200,
        })
    }

    pub fn get_tasks_by_templateid(
        template_input: Uuid,
        token: &str,
    ) -> Result<Response<Vec<TemplatingTask>>, Response<String>> {
        match crate::models::template::Template::check_user_valid(token, template_input) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let conn = &mut back::establish_connection();
        use crate::schema::schema::templating_task::dsl::{templateid, templating_task};

        let results = templating_task
            .filter(templateid.eq(template_input))
            .load::<TemplatingTask>(conn)
            .expect("Couldn't find the template's tasks in the database.");

        return Ok(Response {
            data: results,
            success: true,
            status: 200,
        });
    }

    pub fn delete(id: &str, token: &str) -> Result<Response<String>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let input_uuid: Uuid;

        match StrChange::to_uuid(id) {
            Ok(o) => {
                input_uuid = o;
            }
            Err(e) => {
                println!("{}", status::Custom(Status::BadRequest, e.to_string()).0);
                return Err(Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                });
            }
        }

        let template_from_task = TemplatingTask::get_task_by_id(id).unwrap();
        let conn = &mut back::establish_connection();

        match crate::models::template::Template::check_user_valid(
            token,
            template_from_task.data.templateid.unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        use crate::schema::schema::templating_task::dsl::{templating_task, temptaskid};
        match diesel::delete(templating_task.filter(temptaskid.eq(input_uuid))).execute(conn) {
            Ok(res) => {
                if res.to_string().parse::<i32>().unwrap() > 0 {
                    Ok(Response {
                        success: true,
                        data: format!("{} task deleted.", res.to_string()),
                        status: 200,
                    })
                } else {
                    Ok(Response {
                        success: false,
                        data: "There isn't any task with that uuid.".to_string(),
                        status: 200,
                    })
                }
            }
            Err(e) => {
                return Err(Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                });
            }
        }
    }

    pub fn update(updated_task: UpdatedTemplatingTask, token: &str) -> Response<String> {
        use crate::schema::schema::templating_task::dsl::{
            content, duetime, templating_task, temptaskid,
        };
        let conn = &mut back::establish_connection();

        let updated_content = &updated_task.content;
        let updated_duetime = &updated_task.duetime;

        match crate::models::template::Template::check_user_valid(
            token,
            updated_task.templateid.unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => return e,
        }

        let _data = diesel::update(templating_task.filter(temptaskid.eq(&updated_task.temptaskid)))
            .set((content.eq(updated_content), duetime.eq(updated_duetime)))
            .execute(conn);

        Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        }
    }

    pub fn add(
        token: &str,
        mut new_task: AddTemplatingTask,
    ) -> Result<Response<TemplatingTask>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::templating_task;
        let decoded_token = Auth::decode_token(token.to_string());

        new_task.userid = Some(decoded_token.unwrap().data.user_token.userid);

        Ok(Response {
            success: true,
            data: diesel::insert_into(templating_task::table)
                .values(new_task)
                .get_result(conn)
                .expect("An error occured while saving a new template: {}"),
            status: 200,
        })
    }
}
