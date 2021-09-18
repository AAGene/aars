use crate::packets::game_server_packets::RegisterGameServerData;
use std::env;

pub fn register(data: RegisterGameServerData) {
    let key = env::var("SERVER_KEY").expect("SERVER_KEY must be set");

    if data.secret_key != key {
        log::error!("Server trying to register with wrong secret key, got: {}", data.secret_key)
    } else {
        log::info!("GS {} has the correct secret key", data.game_server_id)
    }

    // TODO: Check database if GS exists

    // TODO: Register the server!

    // TODO: Keep the connection somewhere, right?
}