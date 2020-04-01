use std::net::SocketAddr;

pub struct InPacket(SocketAddr, Vec<u8>);
pub struct OutPacket(Vec<u8>);

impl InPacket {
    pub fn new(addr: SocketAddr, data: Vec<u8>) -> InPacket {
        InPacket(addr, data)
    }
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }
}

impl OutPacket {
    pub fn new(data: Vec<u8>) -> OutPacket {
        OutPacket(data)
    }
}
