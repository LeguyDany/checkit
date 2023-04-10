use crate::models::template::{Template, Weekdays};

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
