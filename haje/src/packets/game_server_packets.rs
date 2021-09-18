use shatigon::network::packet::PacketStream;

pub struct RegisterGameServerData {
    pub secret_key: String,
    pub game_server_id: u8,
    pub additional_game_server_ids: Vec<u8>,
}

pub fn read_register_game_server_packet(mut stream: PacketStream) -> RegisterGameServerData {
    let secret_key = stream.read_string();
    let game_server_id = stream.read_u8();
    let mirrors = stream.read_vec_32();
    log::trace!("Reading GLRegisterGameServerPacket - key {}, gsId {}, mirrors {}", secret_key, game_server_id, mirrors.len());

    RegisterGameServerData {
        secret_key,
        game_server_id,
        additional_game_server_ids: mirrors
    }
}