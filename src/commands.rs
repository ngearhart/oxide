use crate::{serialization::encode_simple_string, store::Store};

#[derive(strum_macros::Display)]
#[derive(strum_macros::EnumString)]
pub enum CommandType {
    PING,
    ECHO
}

impl CommandType {
    fn get_function_to_execute(&self) -> fn(&Vec<&str>, &Store) -> String {
        match self {
            CommandType::PING => ping_execute,
            CommandType::ECHO => echo_execute,
        }
    }

    pub fn execute(&self, args: &Vec<&str>, global_store: &Store) -> String {
        self.get_function_to_execute()(args, global_store)
    }
}

fn ping_execute(_: &Vec<&str>, _: &Store) -> String {
    encode_simple_string("PONG".to_string())
}

fn echo_execute(args: &Vec<&str>, _: &Store) -> String {
    encode_simple_string(args.join(" "))
}
