
use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::commands;

const LINE_TERMINATOR: &'static str = "\r\n";

#[derive(Debug, EnumIter)]
#[derive(PartialEq)]
pub enum DataType {
    SimpleString,
    Error,
    Integer,
    BulkString,
    Array,
}

impl DataType {
    pub fn value(&self) -> char {
        match self {
            DataType::SimpleString => '+',
            DataType::Error => '-',
            DataType::Integer => ':',
            DataType::BulkString => '$',
            DataType::Array => '*',
        }
    } 
}

// Base case - for simple strings, errors
fn deserialize_trivial(message: &String) -> &String {
    message
}

fn deserialize_integer(message: &String) -> i64 {
    return message.parse::<i64>()
        .expect("Received an invalid int message")
}

// fn deserialize_bulk_string(message: &String) -> Vec<&String> {
//     let result: Vec<&String> = Vec::new();
//     for component in message.split(LINE_TERMINATOR) {
//         result.push(deserialize_message(component));
//     }
//     result
// }

fn is_null(message: &String) -> bool {
    message == "$-1\r\n" || message == "*-1\r\n"
}

fn get_message_data_type(message: &String) -> Option<DataType> {
    for data_type in DataType::iter() {
        if message.starts_with(data_type.value()) {
            return Some(data_type)
        }
    }
    None
}

fn get_response_for_payload(message: &String) -> Option<String> {
    let data_type = get_message_data_type(message)
        .expect("Invalid message type");
    let payload = &String::from(&message[1..]);
    if data_type == DataType::SimpleString {
        return Some(commands::Command::from_str(payload)
            .expect("Command unimplemented")
            .execute(payload))
    }
    None
}

fn encode_response(response: String) -> String {
    format!("{}{}{}", DataType::SimpleString.value(), response, LINE_TERMINATOR)
}

pub fn receive_message(message: &String) -> String {
    encode_response(
        get_response_for_payload(message)
            .expect("Invalid data type")
    )
}

#[test] 
fn test_get_message_data_type() {
    assert_eq!(get_message_data_type(&String::from("+test")), Some(DataType::SimpleString));
    assert_eq!(get_message_data_type(&String::from("sedx")), None);
}
