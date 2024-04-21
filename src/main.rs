mod server;
mod commands;
mod serialization;

use env_logger::Builder;
use crate::server::start_server;

fn main() {
    Builder::new()
        .parse_env("LOG_LEVEL")
        .init();
    log::info!("Starting");
    start_server();
}
