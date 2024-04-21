
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

fn deserialize_integer(message: &str) -> i64 {
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
    else if data_type == DataType::Array {
        // We know the array length is the next character(s).
        let mut current_array_message = message.split(LINE_TERMINATOR);
        current_array_message.next();
        for message_item in current_array_message {
            println!("Item: {}", message_item.to_string());
        }
    }
    None
}

pub fn decode_command(command: &String) {
    // Command arrives as an array of bulk strings.
    let data_type = get_message_data_type(command)
        .expect("Invalid message type");
    assert_eq!(data_type, DataType::Array);
    assert!(command.len() > 1);

    let mut bulk_strings = command[1..].split(LINE_TERMINATOR);

    let array_length = deserialize_integer(
        bulk_strings
        .next()
        .expect("Could not find first command")
    );
    println!("Command length: {}", array_length);

    // Get the command body - second element
    // Push ahead by 1 + (length size) + (line terminator size)
    println!("Size: {}", f32::floor(f32::log10(array_length as f32)));
    let mut command_body = &command[
        (2 + f32::floor(f32::log10(array_length as f32)) as usize + LINE_TERMINATOR.len())..
    ];
    let mut command_name: Option<&str> = None;
    let mut args: Vec<&str> = Vec::new();
    for index in 0..array_length {
        let data_type = get_message_data_type(&String::from(command_body))
            .expect("Invalid message type");
        assert_eq!(data_type, DataType::BulkString);
        let string_length = deserialize_integer(
            command_body[1..].split(LINE_TERMINATOR).next()
            .expect("Could not find string length in bulk string")
        );
        // Push ahead by 1 + (length size) + (line terminator size)
        // not sure why I'm using 2
        println!("Size: {}", f32::floor(f32::log10(string_length as f32)));
        command_body = &command_body[
            (2 + f32::floor(f32::log10(string_length as f32)) as usize + LINE_TERMINATOR.len())..
        ];
        if index == 0 {
            command_name = Some(&command_body[..string_length as usize]);
        } else {
            args.push(&command_body[..string_length as usize]);
        }

        // Push ahead by length size + line terminator size
        command_body = &command_body[string_length as usize + LINE_TERMINATOR.len()..];
    }
    println!("Command: {}", command_name.expect("Command name not set"));
    for argument in args {
        println!("Args: {}", argument);
    }
}

// fn convert_bulk_string(bulk_string: &str) -> &str {

// }

fn encode_response(response: String) -> String {
    format!("{}{}{}", DataType::SimpleString.value(), response, LINE_TERMINATOR)
}

pub fn receive_message(message: &String) -> String {
    encode_response(
        get_response_for_payload(message)
            .expect("Invalid data type")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_get_message_data_type() {
        assert_eq!(get_message_data_type(&String::from("+test")), Some(DataType::SimpleString));
        assert_eq!(get_message_data_type(&String::from("sedx")), None);
    }


}
