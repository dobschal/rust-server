use mysql::{params, prelude::*};

use crate::{database::get_connection, dto::UserDto, entity::User};

pub fn get_users() -> Vec<User> {
    println!("Select users from database...");
    let users = get_connection()
        .query_map("SELECT id, name from user", |(id, name)| User { id, name })
        .expect("Could not get users...");
    println!("Users: {:?}", users);
    return users;
}

pub fn add_user(user: UserDto) {
    println!("Add user to database...");
    get_connection()
        .exec_drop(
            "INSERT INTO user (name) VALUES (:name)",
            params! { "name" => user.name },
        )
        .expect("Could not insert user...");
}
