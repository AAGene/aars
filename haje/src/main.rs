extern crate diesel;

mod network;

use log::{info, trace, warn};
use shatigon::network::server::TcpServer;

use self::diesel::prelude::*;
use haje::establish_connection;
use haje::models::models::Account;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("AAGenesis - Rust login sever PoC");

    // TODO: Load config

    // TODO: Connect to PostgreSQL
    use haje::schema::accounts::dsl::*;
    let connection = establish_connection();

    // TODO: Start a server
    let mut server = network::login_server::LoginServer::new();
    server.start();
}
