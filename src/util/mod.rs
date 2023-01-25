use std::{io::Write, net::TcpStream};

pub fn respond_with_success(stream: &mut TcpStream) {
    let contents = "{\"success\": true }".to_owned();
    let length = contents.len();
    stream
        .write_all(
            format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}").as_bytes(),
        )
        .unwrap();
}

pub fn respond_with_bad_request_error(stream: &mut TcpStream, error_message: &str) {
    respond_with_error(stream, error_message, "400 Bad Request");
}

pub fn respond_with_error(stream: &mut TcpStream, error_message: &str, code: &str) {
    let contents = format!("{{\"error\": \"{}\"}}", error_message).to_owned();
    let length = contents.len();
    stream
        .write_all(
            format!("HTTP/1.1 {code}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}").as_bytes(),
        )
        .unwrap();
}
