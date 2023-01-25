use mysql::{params, prelude::*};

use crate::{database::get_connection, dto::UserDto, entity::User};

pub fn get_users() -> Result<Vec<User>, mysql::Error> {
    get_connection().query_map("SELECT id, name from user", |(id, name)| User { id, name })
}

pub fn add_user(user: UserDto) -> Result<(), mysql::Error> {
    get_connection().exec_drop(
        "INSERT INTO user (name) VALUES (:name)",
        params! { "name" => user.name },
    )
}
