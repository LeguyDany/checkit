use crate::models::auth::Auth;
use crate::models::executable_task::{AddExeTask, ExeTask, UpdatedExeTask};
use crate::models::response::Response;
use crate::models::template::Template;
use crate::models::templating_task::TemplatingTask;
use diesel::prelude::*;
use uuid::Uuid;

use rocket::http::Status;
use rocket::response::status;

impl ExeTask {
    // pub fn get_tasks_for_today(token: &str) {
    //     use crate::models::template::Template;
    //
    //     Template::get_template_by_userid(token);
    // }

    pub fn get_task_by_id(id: &str) -> Result<Response<ExeTask>, Response<String>> {
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
        use crate::schema::schema::executable_task::dsl::executable_task;

        let mut task_filtered = executable_task
            .find(input_uuid)
            .load::<ExeTask>(conn)
            .expect("Err loading tasks");

        if task_filtered.len() > 0 {
            let response: ExeTask = task_filtered.remove(0);
            return Ok(Response {
                success: true,
                data: response,
                status: 200,
            });
        }
        Err(Response {
            success: false,
            data: "There isn't any executable task with that id.".to_string(),
            status: 404,
        })
    }

    pub fn get_tasks_by_templateid(
        template_input: &str,
        token: &str,
    ) -> Result<Response<Vec<ExeTask>>, Response<String>> {
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
        use crate::schema::schema::executable_task::dsl::{executable_task, templateid};

        let results = executable_task
            .filter(templateid.eq(template_input_uuid))
            .load::<ExeTask>(conn)
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

        let task_from_id = ExeTask::get_task_by_id(id);

        match task_from_id {
            Ok(o) => {
                let conn = &mut back::establish_connection();

                match crate::models::template::Template::check_user_valid(
                    token,
                    o.data.templateid.unwrap(),
                ) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                use crate::schema::schema::executable_task::dsl::{executable_task, exetaskid};
                match diesel::delete(executable_task.filter(exetaskid.eq(input_uuid))).execute(conn)
                {
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
            Err(e) => return Err(e),
        }
    }

    pub fn update(updated_task: UpdatedExeTask, token: &str) -> Response<String> {
        use crate::schema::schema::executable_task::dsl::{
            checked, content, duetime, executable_task, exetaskid,
        };
        let conn = &mut back::establish_connection();

        let updated_content = &updated_task.content;
        let updated_duetime = &updated_task.duetime;
        let updated_checked = &updated_task.checked;

        match crate::models::template::Template::check_user_valid(
            token,
            updated_task.templateid.unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => return e,
        }

        let _data = diesel::update(executable_task.filter(exetaskid.eq(&updated_task.exetaskid)))
            .set((
                content.eq(updated_content),
                checked.eq(updated_checked),
                duetime.eq(updated_duetime),
            ))
            .execute(conn);

        Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        }
    }

    pub fn add(
        token: &str,
        mut new_task: AddExeTask,
    ) -> Result<Response<ExeTask>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::executable_task;
        let decoded_token = Auth::decode_token(token.to_string());

        new_task.userid = Some(decoded_token.unwrap().data.user_token.userid);

        Ok(Response {
            success: true,
            data: diesel::insert_into(executable_task::table)
                .values(new_task)
                .get_result(conn)
                .expect("An error occured while saving a new template: {}"),
            status: 200,
        })
    }

    pub fn add_exetasks_by_temptask(token: &str) -> Result<Response<String>, Response<String>> {
        let templates_for_today = Template::get_templates_for_today(token).unwrap();

        for temp in templates_for_today.data {
            let task_list: Vec<TemplatingTask>;

            match TemplatingTask::get_tasks_by_templateid(temp.templateid, token) {
                Ok(o) => task_list = o.data,
                Err(e) => return Err(e),
            }

            for template_task in task_list {
                let _data = ExeTask::add(
                    token.clone(),
                    AddExeTask {
                        content: template_task.content,
                        userid: template_task.userid,
                        duetime: template_task.duetime,
                        checked: false,
                        templateid: template_task.templateid,
                    },
                );
            }
        }
        Ok(Response {
            success: true,
            data: "Executable tasks added successfully.".to_string(),
            status: 200,
        })
    }
}
