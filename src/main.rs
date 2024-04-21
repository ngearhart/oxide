mod server;
mod commands;
mod serialization;
mod store;

use env_logger::Builder;
use crate::{server::start_server, store::Store};

fn main() {
    Builder::new()
        .parse_env("LOG_LEVEL")
        .init();
    log::info!("Starting");

    let global_store = &Store::new();

    start_server(global_store);
}
