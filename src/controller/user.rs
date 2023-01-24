use crate::database;
use serde_json::json;
use std::{fs, io::Write, net::TcpStream};

static RETURN_JSON: bool = true;

pub fn fake(stream: &mut TcpStream) {
    println!("fake");
    let contents = "{\"success\": true }".to_owned();
    let length = contents.len();
    stream
        .write_all(
            format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}").as_bytes(),
        )
        .unwrap();
}

//  This controller method loads all users from the MySQL database
//  depending on the RETURN_JSON flag, it returns JSON or HTML
//  the database module holds a MySQL connection pool that gets fetched first
//
pub fn get_users(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let users = database::get_users();
    let contents = if RETURN_JSON {
        json!(users).to_string()
    } else {
        let users_print = users
            .iter()
            .map(|user| format!("<li>{}</li>", user.name.as_ref().unwrap()))
            .collect::<Vec<String>>()
            .join("\r\n");
        fs::read_to_string("static/users.html")
            .unwrap()
            .replace("#_USERS", &users_print)
    };
    let length = contents.len();
    let content_type = if RETURN_JSON {
        "application/json"
    } else {
        "text/html"
    };
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
