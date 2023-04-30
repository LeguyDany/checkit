use crate::models::auth::Auth;
use crate::models::response::Response;
use crate::models::template::{Template, Weekdays};
use uuid::Uuid;

impl Template {
    pub fn check_user_valid(
        token: &str,
        current_template_uuid: Uuid,
    ) -> Result<bool, Response<String>> {
        let current_template_userid = Template::get_template_by_id(current_template_uuid)?
            .data
            .userid
            .ok_or_else(|| Response {
                success: false,
                data: "No template associated with the current user was found.".to_string(),
                status: 404,
            })?;

        let decoded_token = Auth::decode_token(token.to_string())?.data;

        if decoded_token.user_token.userid != current_template_userid {
            return Err(Response {
                success: false,
                data: "Wrong user, please log in with the right account to do this.".to_string(),
                status: 400,
            });
        };

        return Ok(true);
    }
}

impl Weekdays {
    pub fn convert_weekdays_to_array(&self) -> Vec<bool> {
        vec![
            self.monday,
            self.tuesday,
            self.wednesday,
            self.thursday,
            self.friday,
            self.saturday,
            self.sunday,
        ]
    }
}
