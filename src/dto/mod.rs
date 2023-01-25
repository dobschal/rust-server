use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserDto {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub payload: String,
    pub header: String,
    pub http_method: String,
    pub path: String,
}
