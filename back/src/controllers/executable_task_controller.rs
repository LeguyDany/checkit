use crate::helpers::str_helper::StrChange;
use crate::models::executable_task::{AddExeTask, ExeTask, UpdatedExeTask};
use crate::models::response::Response;
use crate::models::template::Template;
use crate::models::templating_task::TemplatingTask;
use diesel::prelude::*;

impl ExeTask {
    pub fn get_task_by_id(id: &str) -> Result<Response<ExeTask>, Response<String>> {
        let input_uuid = StrChange::to_uuid(id)?;

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
        return Err(Response {
            success: false,
            data: "There isn't any executable task with that id.".to_string(),
            status: 404,
        });
    }

    pub fn get_tasks_by_templateid(
        template_input: &str,
        token: &str,
    ) -> Result<Response<Vec<ExeTask>>, Response<String>> {
        let template_input_uuid = StrChange::to_uuid(template_input)?;

        crate::models::template::Template::check_user_valid(token, template_input_uuid)?;

        let conn = &mut back::establish_connection();
        use crate::schema::schema::executable_task::dsl::{executable_task, templateid};

        let results = executable_task
            .filter(templateid.eq(template_input_uuid))
            .load::<ExeTask>(conn)
            .map_err(|_| Response {
                success: false,
                data: "Couldn't find the template's tasks in the database.".to_string(),
                status: 404,
            })?;

        return Ok(Response {
            data: results,
            success: true,
            status: 200,
        });
    }

    pub fn delete(id: &str, token: &str) -> Result<Response<String>, Response<String>> {
        let input_uuid = StrChange::to_uuid(id)?;

        let task_from_id = ExeTask::get_task_by_id(id)?;

        let conn = &mut back::establish_connection();

        crate::models::template::Template::check_user_valid(
            token,
            task_from_id.data.templateid.unwrap(),
        )?;

        use crate::schema::schema::executable_task::dsl::{executable_task, exetaskid};

        let number_of_deleted_executable_task =
            diesel::delete(executable_task.filter(exetaskid.eq(input_uuid)))
                .execute(conn)
                .map_err(|e| Response {
                    success: false,
                    data: format!("Error: {}", e.to_string()),
                    status: 200,
                })?;

        if number_of_deleted_executable_task
            .to_string()
            .parse::<i32>()
            .unwrap()
            > 0
        {
            Ok(Response {
                success: true,
                data: format!(
                    "{} task deleted.",
                    number_of_deleted_executable_task.to_string()
                ),
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

    pub fn update(
        updated_task: UpdatedExeTask,
        token: &str,
    ) -> Result<Response<String>, Response<String>> {
        use crate::schema::schema::executable_task::dsl::{
            checked, content, duetime, executable_task, exetaskid,
        };
        let conn = &mut back::establish_connection();

        let updated_content = &updated_task.content;
        let updated_duetime = &updated_task.duetime;
        let updated_checked = &updated_task.checked;

        crate::models::template::Template::check_user_valid(
            token,
            updated_task.templateid.unwrap(),
        )?;

        let _data = diesel::update(executable_task.filter(exetaskid.eq(&updated_task.exetaskid)))
            .set((
                content.eq(updated_content),
                checked.eq(updated_checked),
                duetime.eq(updated_duetime),
            ))
            .execute(conn);

        return Ok(Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        });
    }

    pub fn add(
        userid: &str,
        mut new_task: AddExeTask,
    ) -> Result<Response<ExeTask>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::executable_task;

        let userid_to_uuid = crate::helpers::str_helper::StrChange::to_uuid(userid).unwrap();
        new_task.userid = Some(userid_to_uuid);

        Ok(Response {
            success: true,
            data: diesel::insert_into(executable_task::table)
                .values(new_task)
                .get_result(conn)
                .expect("An error occured while saving a new template: {}"),
            status: 200,
        })
    }

    pub fn add_exetasks_by_temptask(userid: &str) -> Result<Response<String>, Response<String>> {
        let templates_for_today = Template::get_templates_for_today(userid)?;

        for temp in templates_for_today.data {
            let task_list: Vec<TemplatingTask> =
                TemplatingTask::get_tasks_by_templateid(temp.templateid)?.data;

            for template_task in task_list {
                println!("Template: {:?}", template_task.content);
                ExeTask::add(
                    userid,
                    AddExeTask {
                        content: template_task.content,
                        userid: template_task.userid,
                        duetime: template_task.duetime,
                        checked: false,
                        templateid: template_task.templateid,
                    },
                )?;
            }
        }
        return Ok(Response {
            success: true,
            data: "Executable tasks added successfully.".to_string(),
            status: 200,
        });
    }
}
