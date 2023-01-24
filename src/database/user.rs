use mysql::prelude::Queryable;

use crate::{database::get_connection, entity::User};

pub fn get_users() -> Vec<User> {
    println!("Select users from database...");
    let users = get_connection()
        .query_map("SELECT id, name from user", |(id, name)| User { id, name })
        .expect("Could not get users...");
    println!("Users: {:?}", users);
    return users;
}
