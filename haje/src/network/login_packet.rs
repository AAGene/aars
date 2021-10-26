use byteorder::{ByteOrder, LittleEndian};
use shatigon::network::packet::{Packet, PacketStream, Serializable};

pub struct LoginPacket<T>
where
    T: Serializable,
{
    pub length: u16,
    pub opcode: u16,
    pub level: u16,
    pub stream: PacketStream,
    pub packet: Option<T>,
}

impl<T> LoginPacket<T>
where
    T: Serializable,
{
    pub fn new(mut stream: PacketStream) -> Self {
        let mut packet = LoginPacket {
            opcode: 0,
            level: 0,
            length: 0,
            stream: stream.clone(),
            packet: None,
        };

        packet.read(stream.clone());
        packet
    }
}

impl<T> Serializable for LoginPacket<T>
where
    T: Serializable,
{
    fn write(&self, stream: PacketStream) {}

    fn read(&mut self, mut stream: PacketStream) {
        self.length = stream.read_u16();
        self.opcode = stream.read_u16();

        if self.packet.is_some() {
            let sub = self.packet.as_mut().unwrap();
            sub.read(stream);
        }
    }
}

pub struct EmptyPacket {}

impl EmptyPacket {
    pub fn new() -> EmptyPacket {
        EmptyPacket {}
    }
}

impl Serializable for EmptyPacket {
    fn write(&self, stream: PacketStream) {}

    fn read(&mut self, stream: PacketStream) {
        println!("Empty packet")
    }
}

pub struct GLRegisterGameServerPacket {
    secret_key: String,
    game_server_id: u8,
    additional_game_server_ids: Vec<u8>,
}

impl GLRegisterGameServerPacket {
    pub fn new() -> GLRegisterGameServerPacket {
        GLRegisterGameServerPacket {
            game_server_id: 0,
            additional_game_server_ids: vec![],
            secret_key: "".to_string(),
        }
    }
}

impl Serializable for GLRegisterGameServerPacket {
    fn write(&self, stream: PacketStream) {}

    fn read(&mut self, mut stream: PacketStream) {
        self.secret_key = stream.read_string();
        self.game_server_id = stream.read_u8();
        let mirrors = stream.read_vec_32();
        log::trace!(
            "Reading GLRegisterGameServerPacket - key {}, gsId {}, mirrors {}",
            self.secret_key,
            self.game_server_id,
            mirrors.len()
        );
    }
}
