use crate::models::auth::Auth;
use crate::models::response::Response;
use uuid::Uuid;

use crate::models::template::{AddTemplate, Template, UpdatedTemplate};
use diesel::prelude::*;

use rocket::http::Status;
use rocket::response::status;

impl Template {
    pub fn get_template_by_id(id: Uuid) -> Response<Template> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::template::dsl::template;

        let mut template_filtered = template
            .find(id)
            .load::<Template>(conn)
            .expect("Err loading posts");

        let response: Template = template_filtered.remove(0);
        Response {
            success: true,
            data: response,
            status: 200,
        }
    }

    pub fn get_template_by_userid(
        token: &str,
    ) -> Result<Response<Vec<Template>>, Response<String>> {
        let conn = &mut back::establish_connection();
        use crate::schema::schema::template::dsl::{template, userid as tuserid};

        let decoded_token;

        match Auth::decode_token(token.to_string()) {
            Ok(o) => decoded_token = o.data,
            Err(e) => return Err(e),
        }

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

        match Template::check_user_valid(token, input_uuid) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        use crate::schema::schema::template::dsl::{template, templateid};
        match diesel::delete(template.filter(templateid.eq(input_uuid))).execute(conn) {
            Ok(res) => {
                if res.to_string().parse::<i32>().unwrap() > 0 {
                    Ok(Response {
                        success: true,
                        data: format!("{} template deleted.", res.to_string()),
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
            Err(e) => {
                return Err(Response {
                    success: false,
                    data: e.to_string(),
                    status: 400,
                });
            }
        }
    }

    pub fn update(updated_template: UpdatedTemplate, token: &str) -> Response<String> {
        use crate::schema::schema::template::dsl::{template, templateid, templatename, weekdays};
        let conn = &mut back::establish_connection();

        let updated_name = &updated_template.template_name;
        let updated_weekdays = &updated_template.weekdays;

        match Template::check_user_valid(token, updated_template.templateid) {
            Ok(_) => {}
            Err(e) => return e,
        }

        let _data = diesel::update(template.filter(templateid.eq(&updated_template.templateid)))
            .set((
                templatename.eq(updated_name),
                weekdays.eq(updated_weekdays.convert_weekdays_to_array()),
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
        mut new_template: AddTemplate,
    ) -> Result<Response<Template>, Response<String>> {
        let conn = &mut back::establish_connection();

        use crate::schema::schema::template;
        let decoded_token = Auth::decode_token(token.to_string());

        new_template.userid = Some(decoded_token.unwrap().data.user_token.userid);

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
