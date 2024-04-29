use crate::serialization::{encode_simple_string, NULL};
use crate::store::{global_store_get, global_store_set};

#[derive(strum_macros::Display)]
#[derive(strum_macros::EnumString)]
pub enum CommandType {
    PING,
    ECHO,
    SET,
    GET
}

impl CommandType {
    fn get_function_to_execute(&self) -> fn(&Vec<&str>) -> String {
        match self {
            CommandType::PING => ping_execute,
            CommandType::ECHO => echo_execute,
            CommandType::SET => set_execute,
            CommandType::GET => get_execute,
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
    encode_simple_string(args.join(" "))
}

fn set_execute(args: &Vec<&str>) -> String {
    assert_eq!(args.len(), 2);
    let key = args.get(0).expect("Missing key");
    let val = args.get(1).expect("Missing value");
    global_store_set(key.to_string(), val.to_string());
    encode_simple_string("OK".to_string())
}

fn get_execute(args: &Vec<& str>) -> String {
    assert_eq!(args.len(), 1);
    let key = args.get(0).expect("Missing key");
    let result = global_store_get(key.to_string());
    if result == NULL {
        return NULL.to_owned();
    }
    encode_simple_string(result)
}
