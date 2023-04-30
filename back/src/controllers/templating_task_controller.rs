use crate::models::auth::Auth;
use crate::models::response::Response;
use uuid::Uuid;

use crate::models::templating_task::{AddTemplatingTask, TemplatingTask, UpdatedTemplatingTask};
use diesel::prelude::*;

impl TemplatingTask {
    pub fn get_task_by_id(id: &str) -> Result<Response<TemplatingTask>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let input_uuid = StrChange::to_uuid(id)?;

        let conn = &mut back::establish_connection();
        use crate::schema::schema::templating_task::dsl::templating_task;

        let mut task_filtered = templating_task
            .find(input_uuid)
            .load::<TemplatingTask>(conn)
            .expect("Err loading tasks");

        if task_filtered.len() == 0 {
            return Err(Response {
                success: false,
                data: "Could not find the template task.".to_string(),
                status: 404,
            });
        }

        let response: TemplatingTask = task_filtered.remove(0);
        Ok(Response {
            success: true,
            data: response,
            status: 200,
        })
    }

    pub fn get_tasks_by_templateid(
        template_input: Uuid,
    ) -> Result<Response<Vec<TemplatingTask>>, Response<String>> {
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

        let input_uuid = StrChange::to_uuid(id)?;

        let template_from_task = TemplatingTask::get_task_by_id(id)?;
        let conn = &mut back::establish_connection();

        crate::models::template::Template::check_user_valid(
            token,
            template_from_task.data.templateid.unwrap(),
        )?;

        use crate::schema::schema::templating_task::dsl::{templating_task, temptaskid};

        let number_of_deleted_templating_task =
            diesel::delete(templating_task.filter(temptaskid.eq(input_uuid)))
                .execute(conn)
                .map_err(|e| Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                })?;

        if number_of_deleted_templating_task
            .to_string()
            .parse::<i32>()
            .unwrap()
            > 0
        {
            return Ok(Response {
                success: true,
                data: format!(
                    "{} task deleted.",
                    number_of_deleted_templating_task.to_string()
                ),
                status: 200,
            });
        }

        return Ok(Response {
            success: false,
            data: "There isn't any task with that uuid.".to_string(),
            status: 200,
        });
    }

    pub fn update(
        updated_task: UpdatedTemplatingTask,
        token: &str,
    ) -> Result<Response<String>, Response<String>> {
        use crate::schema::schema::templating_task::dsl::{
            content, duetime, templating_task, temptaskid,
        };
        let conn = &mut back::establish_connection();

        let updated_content = &updated_task.content;
        let updated_duetime = &updated_task.duetime;

        crate::models::template::Template::check_user_valid(
            token,
            updated_task.templateid.unwrap(),
        )?;

        let _data = diesel::update(templating_task.filter(temptaskid.eq(&updated_task.temptaskid)))
            .set((content.eq(updated_content), duetime.eq(updated_duetime)))
            .execute(conn);

        return Ok(Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        });
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
