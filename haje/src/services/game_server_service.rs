use super::database;
use crate::models::models::GameServer;
use crate::packets::game_server_packets::RegisterGameServerData;
use crate::schema::game_servers;
use diesel::prelude::*;
use diesel::PgConnection;
use std::env;

/// Registers (aka marks as online) a game server for players to connect to.
pub fn register(data: RegisterGameServerData) {
    let key = env::var("SERVER_KEY").expect("SERVER_KEY must be set");

    if data.secret_key != key {
        log::error!(
            "GS trying to register with wrong secret key, got: {}",
            data.secret_key
        );

        return;
    }

    let game_server_data = get_game_server(
        &(data.game_server_id as i32),
        database::get_connection().as_ref().unwrap(),
    );

    log::info!(
        "Registering GS '{}' ({}) - {}:{}",
        game_server_data.name,
        game_server_data.id,
        game_server_data.ip,
        game_server_data.port
    );

    // TODO: Register the server!

    // TODO: Keep the connection somewhere, right?
}

/// Gets a GameServer object by querying the database for it via ID. If not found, errors out. TODO: Use optional for better safety
fn get_game_server(gsid: &i32, db: &PgConnection) -> GameServer {
    use crate::schema::game_servers::dsl::*;

    let result: GameServer = game_servers
        .filter(id.eq(gsid))
        .first::<GameServer>(db)
        .expect("Game Server not found");

    return result;
}
