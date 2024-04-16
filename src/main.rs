

fn main() {
    println!("Starting");
    server::start_server();
}

mod comms {
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
        format!("{}{}{}", DataType::SimpleString.value(), response, "\r\n")
    }

    pub fn receive_message(message: &String) -> String {
        encode_response(
            get_response_for_payload(message)
                .expect("Invalid data type")
        )
    }
}

mod commands {

    #[derive(strum_macros::Display)]
    #[derive(strum_macros::EnumString)]
    pub enum Command {
        PING,
        ECHO
    }

    impl Command {
        fn get_function_to_execute(&self) -> fn(&String) -> String {
            match self {
                Command::PING => ping_execute,
                Command::ECHO => echo_execute,
            }
        }

        pub fn execute(&self, payload: &String) -> String {
            self.get_function_to_execute()(payload)
        }
    }

    fn ping_execute(_: &String) -> String {
        "PONG".to_string()
    }

    fn echo_execute(payload: &String) -> String {
        payload.to_string()
    }
}

mod server {
    use std::net::TcpListener;

    pub fn start_server() {
        let listener = TcpListener::bind("0.0.0.0:6379").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::ingest_message;
    use crate::comms::get_message_data_type;
    use crate::comms::DataType;

    #[test] 
    fn test_get_message_data_type() {
        assert_eq!(get_message_data_type(&String::from("+test")), Some(DataType::SimpleString));
        assert_eq!(get_message_data_type(&String::from("sedx")), None);
    }

    #[test] 
    fn test_ingest_message() {
        ingest_message(&String::from("hi"));
    }
}