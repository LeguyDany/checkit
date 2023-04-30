use crate::models::response::Response;
use uuid::Uuid;

pub struct StrChange();

impl StrChange {
    /// Converts a string to an uuid.
    ///
    /// # Example
    ///
    /// ```
    /// use string_helper::str_change;
    ///
    /// let input_uuid = str_change::to_uuid(id)?;
    /// ```
    pub fn to_uuid(to_convert: &str) -> Result<Uuid, Response<String>> {
        match to_convert.parse::<Uuid>() {
            Ok(res) => Ok(res),
            Err(e) => Err(Response {
                success: false,
                data: e.to_string(),
                status: 400,
            }),
        }
    }
}
