use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}
