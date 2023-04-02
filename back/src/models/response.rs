use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub data: T,
}

// pub struct ResponseWithStatus<T> {
//     pub status: u16,
//     pub content: Response<T>
// }