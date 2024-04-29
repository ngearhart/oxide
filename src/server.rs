
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};
use threadpool::ThreadPool;
use crate::serialization::{decode_command, Command};


const THREAD_COUNT: usize = 5; 


pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();

    let thread_pool = ThreadPool::new(THREAD_COUNT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.fill_buf().unwrap().to_vec();

    buf_reader.consume(http_request.len());

    // let response = "*1\r\n$4\r\npong\r\n";
    
    // let response = receive_message(&String::from_utf8(http_request.clone())
    //     .expect("Could not decode"));
    let raw_command = String::from_utf8(http_request.clone()).expect("Could not decode to utf8");
    let mut response_list: Vec<String> = Vec::new();
    let mut remaining_body = raw_command;
    while remaining_body.len() > 0 {
        let command: &mut Command = &mut Command::new();
        let old_body = remaining_body.clone();
        remaining_body = decode_command(
            &old_body,
            command
        );
        // let response = "+PONG\r\n";
        let response = command.execute();
        response_list.push(response);
    }
    stream.write_all(response_list.join("").as_bytes()).unwrap();
    log::debug!("Request: {:#?}", String::from_utf8(http_request));
    log::debug!("Response: {:#?}", response_list.join(""));
}
