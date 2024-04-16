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
