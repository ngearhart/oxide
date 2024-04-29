use crate::serialization::{encode_array, encode_bulk_string, encode_simple_string, NULL, OK};
use crate::store::{global_config_get, global_config_get_keys, global_store_get, global_store_set};

/// Enum of commands that this Redis server can process.
/// Add a new entry to this enum to support additional commands.
#[derive(strum_macros::Display)]
#[derive(strum_macros::EnumString)]
pub enum CommandType {
    PING,
    ECHO,
    SET,
    GET,
    CONFIG
}

impl CommandType {
    fn get_function_to_execute(&self) -> fn(&Vec<&str>) -> String {
        match self {
            CommandType::PING => ping_execute,
            CommandType::ECHO => echo_execute,
            CommandType::SET => set_execute,
            CommandType::GET => get_execute,
            CommandType::CONFIG => config_execute,
        }
    }

    pub fn execute(&self, args: &Vec<&str>) -> String {
        self.get_function_to_execute()(args)
    }
}

fn ping_execute<'a>(_: &Vec<&str>) -> String {
    encode_simple_string("PONG".to_string())
}

fn echo_execute<'a>(args: &Vec<&str>) -> String {
    encode_bulk_string(args.join(" "))
}

fn set_execute(args: &Vec<&str>) -> String {
    assert_eq!(args.len(), 2);
    let key = args.get(0).expect("Missing key");
    let val = args.get(1).expect("Missing value");
    global_store_set(key.to_string(), val.to_string());
    encode_simple_string(OK.to_owned())
}

fn get_execute(args: &Vec<& str>) -> String {
    assert_eq!(args.len(), 1);
    let key = args.get(0).expect("Missing key");
    let result = global_store_get(key.to_string());
    if result == NULL {
        return NULL.to_owned();
    }
    encode_bulk_string(result)
}

fn config_execute(args: &Vec<& str>) -> String {
    if args.get(0).expect("CONFIG command sent without clarifier").to_string() == "GET" {
        return config_get(args);
    }
    log::debug!("Received CONFIG SET command (unsupported). Blindly replying with OK");
    encode_simple_string(OK.to_owned())
}

fn config_get(args: &Vec<& str>) -> String {
    let mut response: Vec<String> = Vec::new();
    for arg in &args[1..] {
        let keys = global_config_get_keys(arg.to_string());
        for key in keys {
            response.push(key.to_string());
            response.push(global_config_get(key.to_string()));
        }
    }
    encode_array(response)
}
