use std::{fs, io::Write, net::TcpStream};

use crate::database;

pub fn get_users(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let mut contents = fs::read_to_string("static/users.html").unwrap();
    let users = database::get_users()
        .iter()
        .map(|user| format!("<li>{}</li>", user.name.as_ref().unwrap()))
        .collect::<Vec<String>>()
        .join("\r\n");
    contents = contents.replace("#_USERS", &users);
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
