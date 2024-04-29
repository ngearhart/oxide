
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::str::FromStr;

use crate::commands::CommandType;

const LINE_TERMINATOR: &'static str = "\r\n";
pub const NULL: &'static str = "$-1\r\n";
pub const OK: &'static str = "OK";

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

pub struct Command<'a> {
    name: Option<&'a str>,
    args: Vec<&'a str>
}

impl<'a> Command<'a> {
    pub fn new() -> Command<'a> {
        Command {
            name: None,
            args: Vec::new()
        }
    }

    pub fn set_name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    pub fn add_arg(&mut self, arg: &'a str) {
        self.args.push(arg);
    }

    pub fn execute(&mut self) -> String {
        CommandType::from_str(
            &self.name.expect("Command cannot be null").to_uppercase()
        ).expect(&format!("Unknown command: {}", self.name.unwrap())).execute(&self.args)
    }
}

fn deserialize_integer(message: &str) -> i64 {
    return message.parse::<i64>()
        .expect("Received an invalid int message")
}

fn get_message_data_type(message: &String) -> Option<DataType> {
    for data_type in DataType::iter() {
        if message.starts_with(data_type.value()) {
            return Some(data_type)
        }
    }
    None
}

pub fn decode_command<'a>(command: &'a String, result: &mut Command<'a>) {
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
    log::debug!("Command length: {}", array_length);

    // Get the command body - second element
    // Push ahead by 1 + (length size) + (line terminator size)
    let mut command_body = &command[
        (2 + f32::floor(f32::log10(array_length as f32)) as usize + LINE_TERMINATOR.len())..
    ];
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
        command_body = &command_body[
            (2 + f32::floor(f32::log10(string_length as f32)) as usize + LINE_TERMINATOR.len())..
        ];
        if index == 0 {
            result.set_name(&command_body[..string_length as usize]);
        } else {
            result.add_arg(&command_body[..string_length as usize]);
        }

        // Push ahead by length size + line terminator size
        command_body = &command_body[string_length as usize + LINE_TERMINATOR.len()..];
    }
    log::debug!("Command: {}", result.name.expect("Command should not be None"));
    for argument in &result.args {
        log::debug!("Args: {}", argument);
    }
}

// fn convert_bulk_string(bulk_string: &str) -> &str {

// }

pub fn encode_simple_string(response: String) -> String {
    format!("{}{}{}", DataType::SimpleString.value(), response, LINE_TERMINATOR)
}

pub fn encode_bulk_string(response: String) -> String {
    format!("{}{}{}{}{}",
        DataType::BulkString.value(),
        response.len(),
        LINE_TERMINATOR,
        response,
        LINE_TERMINATOR
    )
}

pub fn encode_array(response: Vec<String>) -> String {
    format!("{}{}{}{}",
        DataType::Array.value(),
        response.len(),
        LINE_TERMINATOR,
        response.iter().map(|item| encode_bulk_string(item.to_string())).collect::<Vec<String>>().join(""),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test] 
    fn test_get_message_data_type() {
        assert_eq!(get_message_data_type(&String::from("+test")), Some(DataType::SimpleString));
        assert_eq!(get_message_data_type(&String::from("sedx")), None);
    }


    #[rstest]
    #[case("*1\r\n$4\r\nping\r\n", Some("ping"), 0)]
    #[case("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n", Some("echo"), 1)]
    #[case("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n", Some("get"), 1)]
    fn test_decode_command(#[case] input: &str, #[case] expected_command_name: Option<&str>, #[case] expected_arg_length: usize) {
        let input = &String::from(input);
        let command = &mut Command::new();
        decode_command(input, command);

        if expected_command_name.is_none() {
            assert!(command.name.is_none());
        } else {
            assert!(command.name.is_some());
            assert_eq!(command.name, expected_command_name);
        }
        assert_eq!(command.args.len(), expected_arg_length);
    }
}
