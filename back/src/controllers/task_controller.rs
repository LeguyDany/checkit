use crate::models::auth::Auth;
use crate::models::response::Response;
use uuid::Uuid;

use crate::models::task::{AddTask, Task, UpdatedTask};
use diesel::prelude::*;

use rocket::http::Status;
use rocket::response::status;

impl Task {
    pub fn get_task_by_id(id: &str) -> Result<Response<Task>, Response<String>> {
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
        use crate::schema::schema::task::dsl::task;

        let mut task_filtered = task
            .find(input_uuid)
            .load::<Task>(conn)
            .expect("Err loading tasks");

        let response: Task = task_filtered.remove(0);
        Ok(Response {
            success: true,
            data: response,
            status: 200,
        })
    }

    pub fn get_tasks_by_templateid(
        template_input: &str,
        token: &str,
    ) -> Result<Response<Vec<Task>>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let template_input_uuid: Uuid;

        match StrChange::to_uuid(template_input) {
            Ok(o) => {
                template_input_uuid = o;
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

        match crate::models::template::Template::check_user_valid(token, template_input_uuid) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let conn = &mut back::establish_connection();
        use crate::schema::schema::task::dsl::{task, templateid};

        let results = task
            .filter(templateid.eq(template_input_uuid))
            .load::<Task>(conn)
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

        let template_from_task = Task::get_task_by_id(id).unwrap();
        let conn = &mut back::establish_connection();

        match crate::models::template::Template::check_user_valid(
            token,
            template_from_task.data.templateid.unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        use crate::schema::schema::task::dsl::{task, taskid};
        match diesel::delete(task.filter(taskid.eq(input_uuid))).execute(conn) {
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

    pub fn update(updated_task: UpdatedTask, token: &str) -> Response<String> {
        use crate::schema::schema::task::dsl::{checked, content, task, taskid};
        let conn = &mut back::establish_connection();

        let updated_content = &updated_task.content;
        let updated_checked = &updated_task.checked;

        match crate::models::template::Template::check_user_valid(
            token,
            updated_task.templateid.unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => return e,
        }

        let _data = diesel::update(task.filter(taskid.eq(&updated_task.taskid)))
            .set((content.eq(updated_content), checked.eq(updated_checked)))
            .execute(conn);

        Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        }
    }

    pub fn add(token: &str, mut new_task: AddTask) -> Result<Response<Task>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::task;
        let decoded_token = Auth::decode_token(token.to_string());

        new_task.userid = Some(decoded_token.unwrap().data.user_token.userid);

        Ok(Response {
            success: true,
            data: diesel::insert_into(task::table)
                .values(new_task)
                .get_result(conn)
                .expect("An error occured while saving a new template: {}"),
            status: 200,
        })
    }
}
