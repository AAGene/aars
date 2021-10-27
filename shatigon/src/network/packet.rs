use byteorder::{ByteOrder, LittleEndian};
use std::str;

/**
  A packet transits over the network and serves as communication between the server and client
*/
pub trait Packet: Serializable {
    fn level(&self) -> u16 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
}

/**
  Represents an object that can be written/read inside of network packets
*/
pub trait Serializable: Sized {
    fn write(&self, _stream: PacketStream) {}
    fn read(&mut self, _stream: PacketStream) {}
}

pub struct PacketStream {
    pub raw: Vec<u8>,
    pub idx: u16,
}

impl PacketStream {
    pub fn clone(&mut self) -> PacketStream {
        PacketStream {
            idx: self.idx,
            raw: self.raw.clone(),
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let result = self.raw[self.idx as usize];
        self.idx += 1;
        return result;
    }

    pub fn read_u16(&mut self) -> u16 {
        let result = LittleEndian::read_u16(&self.raw[self.idx as usize..=(self.idx + 1) as usize]);
        self.idx += 2;
        return result;
    }

    pub fn read_u32(&mut self) -> u32 {
        let result = LittleEndian::read_u32(&self.raw[self.idx as usize..=(self.idx + 3) as usize]);
        self.idx += 4;
        return result;
    }

    pub fn read_string(&mut self) -> String {
        let size = self.read_u16();
        let string = str::from_utf8(&self.raw[self.idx as usize..(self.idx + size) as usize]);
        self.idx += size;
        return string.unwrap().to_string();
    }

    pub fn read_vec_32(&mut self) -> Vec<u8> {
        let len = self.read_u32();
        self.read_vec(len as u16)
    }

    pub fn read_vec(&mut self, len: u16) -> Vec<u8> {
        if len == 0 {
            return vec![];
        }

        let res = &self.raw[self.idx as usize..(self.idx + len) as usize];
        self.idx += len;
        return res.to_vec();
    }
}
