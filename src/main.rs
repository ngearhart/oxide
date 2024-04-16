mod server;
mod commands;
mod serialization;

use crate::server::start_server;

fn main() {
    println!("Starting");
    start_server();
}
