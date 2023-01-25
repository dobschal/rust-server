use crate::{
    database::{self},
    dto::UserDto,
    util::respond_with_success,
    Request,
};
use serde_json::{from_str, json};
use std::{io::Write, net::TcpStream};

// TODO: instead of using expect, handle with Result

pub fn add_user(request: &mut Request, stream: &mut TcpStream) {
    let user_dto: UserDto =
        from_str(&request.payload).expect("Could not parse requst payload to UserDto");
    database::add_user(user_dto);
    respond_with_success(stream);
}

pub fn fake(_: &mut Request, stream: &mut TcpStream) {
    println!("fake");
    respond_with_success(stream);
}

pub fn get_users(_request: &mut Request, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let users = database::get_users();
    let contents = json!(users).to_string();
    let length = contents.len();
    let content_type = "application/json";
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
