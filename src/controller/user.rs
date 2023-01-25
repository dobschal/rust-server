use crate::{
    database::{self},
    dto::UserDto,
    util::{
        respond_with_bad_request_error, respond_with_content, respond_with_error,
        respond_with_success,
    },
    Request,
};
use serde_json::{from_str, json, Error};
use std::net::TcpStream;

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
    let get_users_result = database::get_users();
    if get_users_result.is_err() {
        return respond_with_error(stream, "Could not get users", "500 Internal Server Error");
    }
    respond_with_content(stream, json!(get_users_result.unwrap()).to_string());
}
