use crate::models::auth::Auth;
use crate::models::response::Response;
use crate::models::template::{Template, Weekdays};
use uuid::Uuid;

impl Template {
    pub fn convert_array_to_weekdays(input_weekdays: Vec<bool>) -> Weekdays {
        let weekdays = Weekdays {
            monday: input_weekdays[0],
            tuesday: input_weekdays[1],
            wednesday: input_weekdays[2],
            thursday: input_weekdays[3],
            friday: input_weekdays[4],
            saturday: input_weekdays[5],
            sunday: input_weekdays[6],
        };

        weekdays
    }

    pub fn check_user_valid(
        token: &str,
        current_template_uuid: Uuid,
    ) -> Result<bool, Response<String>> {
        let current_template_userid = Template::get_template_by_id(current_template_uuid)
            .data
            .userid
            .unwrap();
        let decoded_token = Auth::decode_token(token.to_string()).unwrap().data;

        if decoded_token.user_token.userid != current_template_userid {
            return Err(Response {
                success: false,
                data: "Wrong user, please log in with the right account to do this.".to_string(),
                status: 400,
            });
        };

        Ok(true)
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
