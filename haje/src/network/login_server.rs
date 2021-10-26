use crate::network::login_packet::{GLRegisterGameServerPacket, LoginPacket};
use byteorder::{ByteOrder, LittleEndian};
use shatigon::network::packet::{PacketStream, Serializable};
use shatigon::network::server::TcpServer;
use std::net::TcpListener;

use haje::packets::game_server_packets::read_register_game_server_packet;
use haje::services::game_server_service;

pub struct LoginServer {}

impl TcpServer for LoginServer {
    fn on_receive(&mut self, mut buf: &Vec<u8>) {
        let mut stream = PacketStream {
            raw: buf.clone(),
            idx: 0,
        };

        let length = stream.read_u16();
        let opcode = stream.read_u16();
        log::trace!("Size: {}, Type: {}", length, opcode);

        match opcode {
            0 => game_server_service::register(read_register_game_server_packet(stream.clone())),
            _ => return log::warn!("Unhandled packet type: {}", opcode),
        };
    }
}

impl LoginServer {
    pub fn new() -> LoginServer {
        LoginServer {}
    }
}
