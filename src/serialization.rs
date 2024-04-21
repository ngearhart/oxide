
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

pub struct Command<'a> {
    name: &'a str,
    args: Vec<&'a str>
}

impl<'a> Command<'a> {
    pub fn new() -> Command<'a> {
        Command {
            name: "",
            args: Vec::new()
        }
    }

    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    pub fn add_arg(&mut self, arg: &'a str) {
        self.args.push(arg);
    }
}

fn deserialize_integer(message: &str) -> i64 {
    return message.parse::<i64>()
        .expect("Received an invalid int message")
}

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

pub fn decode_command<'a>(command: &'a String, mut result: Command<'a>) {
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
    log::debug!("Command: {}", result.name);
    for argument in result.args {
        log::debug!("Args: {}", argument);
    }
}

// fn convert_bulk_string(bulk_string: &str) -> &str {

// }

fn encode_response(response: String) -> String {
    format!("{}{}{}", DataType::SimpleString.value(), response, LINE_TERMINATOR)
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
