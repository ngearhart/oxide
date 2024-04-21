
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

use crate::serialization::decode_command;

pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.fill_buf().unwrap().to_vec();

    buf_reader.consume(http_request.len());

    // let response = "*1\r\n$4\r\npong\r\n";

    // let response = receive_message(&String::from_utf8(http_request.clone())
    //     .expect("Could not decode"));
    decode_command(&String::from_utf8(http_request.clone()).expect("Could not decode to utf8"));
    let response = "+PONG\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", String::from_utf8(http_request));

}
