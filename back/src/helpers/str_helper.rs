use uuid::Uuid;

pub struct StrChange();

impl StrChange {

    /// Converts a string to an uuid.
    /// 
    /// # Example
    /// 
    /// ```
    /// #[path = "../helpers/str_helper.rs"] pub mod string_helper;
    /// use string_helper::str_change;
    ///
    /// let mut response = Vec::new();
    /// let input_uuid: Uuid;
    /// 
    /// match str_change::to_uuid(id) {
    ///    Ok(o) => {
    ///        input_uuid = o;
    ///    }
    ///    Err(x) => {
    ///        response.push("Error".to_owned());
    ///        response.push(x);
    ///        return response;
    ///    }
    /// }
    /// ```
    pub fn to_uuid(to_convert:&str) -> Result<Uuid, String> {
        match to_convert.parse::<Uuid>() {
            Ok(res) => {
                Ok(res)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
}