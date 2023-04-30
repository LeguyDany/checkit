use crate::models::auth::Auth;
use crate::models::response::Response;
use uuid::Uuid;

use crate::models::template::{AddTemplate, Template, UpdatedTemplate};
use diesel::prelude::*;

impl Template {
    pub fn get_templates_for_today(
        userid: &str,
    ) -> Result<Response<Vec<Template>>, Response<String>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::template::dsl::{template, userid as tuserid};

        use chrono::{Datelike, Local};
        let userid_to_uuid = crate::helpers::str_helper::StrChange::to_uuid(userid)?;

        let today = Local::now();
        let today_to_number = today.weekday().number_from_monday() - 1;
        let results = template
            .filter(tuserid.eq(userid_to_uuid))
            .load::<Template>(conn)
            .expect("Couldn't find the user's template in the database.");

        let filtered_results = results
            .into_iter()
            .filter(|temp| temp.weekdays.get(today_to_number as usize) == Some(&Some(true)))
            .collect::<Vec<_>>();

        return Ok(Response {
            data: filtered_results,
            success: true,
            status: 200,
        });
    }

    pub fn get_template_by_id(id: Uuid) -> Result<Response<Template>, Response<String>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::template::dsl::template;

        let mut template_filtered = template
            .find(id)
            .load::<Template>(conn)
            .expect("Err loading posts");

        if template_filtered.len() == 0 {
            return Err(Response {
                success: false,
                data: "Template was not found.".to_string(),
                status: 404,
            });
        }
        let response: Template = template_filtered.remove(0);
        Ok(Response {
            success: true,
            data: response,
            status: 200,
        })
    }

    pub fn get_template_by_userid(
        token: &str,
    ) -> Result<Response<Vec<Template>>, Response<String>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::template::dsl::{template, userid as tuserid};

        let decoded_token = Auth::decode_token(token.to_string())?.data;

        let results = template
            .filter(tuserid.eq(decoded_token.user_token.userid))
            .load::<Template>(conn)
            .expect("Couldn't find the user's template in the database.");

        return Ok(Response {
            data: results,
            success: true,
            status: 200,
        });
    }

    pub fn delete(id: &str, token: &str) -> Result<Response<String>, Response<String>> {
        use crate::helpers::str_helper::StrChange;

        let input_uuid = StrChange::to_uuid(id)?;

        let conn = &mut back::establish_connection();
        Template::check_user_valid(token, input_uuid)?;

        use crate::schema::schema::template::dsl::{template, templateid};

        let number_of_deleted_templates =
            diesel::delete(template.filter(templateid.eq(input_uuid)))
                .execute(conn)
                .map_err(|e| Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                })?;

        if number_of_deleted_templates
            .to_string()
            .parse::<i32>()
            .unwrap()
            > 0
        {
            Ok(Response {
                success: true,
                data: format!(
                    "{} template deleted.",
                    number_of_deleted_templates.to_string()
                ),
                status: 200,
            })
        } else {
            Ok(Response {
                success: false,
                data: "There isn't any template with that uuid.".to_string(),
                status: 200,
            })
        }
    }

    pub fn update(
        updated_template: UpdatedTemplate,
        token: &str,
    ) -> Result<Response<String>, Response<String>> {
        use crate::schema::schema::template::dsl::{template, templateid, templatename, weekdays};
        let conn = &mut back::establish_connection();

        let updated_name = &updated_template.template_name;
        let updated_weekdays = &updated_template.weekdays;

        Template::check_user_valid(token, updated_template.templateid)?;

        let _data = diesel::update(template.filter(templateid.eq(&updated_template.templateid)))
            .set((
                templatename.eq(updated_name),
                weekdays.eq(updated_weekdays.convert_weekdays_to_array()),
            ))
            .execute(conn);

        Ok(Response {
            success: true,
            data: "Template updated".to_string(),
            status: 200,
        })
    }

    pub fn add(
        token: &str,
        mut new_template: AddTemplate,
    ) -> Result<Response<Template>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::template;
        let decoded_token = Auth::decode_token(token.to_string())?.data;

        new_template.userid = Some(decoded_token.user_token.userid);

        Ok(Response {
            success: true,
            data: diesel::insert_into(template::table)
                .values(new_template)
                .get_result(conn)
                .expect("An error occured while saving a new template: {}"),
            status: 200,
        })
    }
}
