extern crate diesel;

mod network;

use log::info;
use shatigon::network::server::TcpServer;

use haje::services::database;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("AAGenesis - Rust login sever PoC");

    // TODO: Load config

    unsafe {
        database::init();
    }

    // TODO: Start a server
    let mut server = network::login_server::LoginServer::new();
    server.start();
}
