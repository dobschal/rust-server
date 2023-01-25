use crate::{
    database::{self},
    dto::UserDto,
    util::{respond_with_bad_request_error, respond_with_error, respond_with_success},
    Request,
};
use serde_json::{from_str, json, Error};
use std::{io::Write, net::TcpStream};

pub fn add_user(request: &mut Request, stream: &mut TcpStream) {
    let parse_user_result: Result<UserDto, Error> = from_str::<UserDto>(&request.payload);
    if parse_user_result.is_err() {
        return respond_with_bad_request_error(stream, "Please request with valid user data.");
    }
    let add_user_result = database::add_user(parse_user_result.unwrap());
    if add_user_result.is_err() {
        return respond_with_error(
            stream,
            "Could not insert user.",
            "500 Internal Server Error",
        );
    }
    respond_with_success(stream);
}

pub fn fake(_: &mut Request, stream: &mut TcpStream) {
    println!("fake");
    respond_with_success(stream);
}

pub fn get_users(_request: &mut Request, stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let get_users_result = database::get_users();
    if get_users_result.is_err() {
        return respond_with_error(stream, "Could not get users", "500 Internal Server Error");
    }
    let contents = json!(get_users_result.unwrap()).to_string();
    let length = contents.len();
    let content_type = "application/json";
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
