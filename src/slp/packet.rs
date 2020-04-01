use std::net::SocketAddr;


pub type Packet = Vec<u8>;

#[derive(Debug)]
pub struct InPacket(Packet, SocketAddr);

#[derive(Debug)]
pub struct OutPacket(Packet);

impl InPacket {
    pub fn new(addr: SocketAddr, data: Packet) -> InPacket {
        InPacket(data, addr)
    }
    pub fn bytes(&self) -> &Packet {
        &self.0
    }
}

impl Into<Packet> for InPacket {
    fn into(self) -> Packet {
        self.0
    }
}

impl AsRef<[u8]> for InPacket {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl OutPacket {
    pub fn new(data: Packet) -> OutPacket {
        OutPacket(data)
    }
}

impl Into<Packet> for OutPacket {
    fn into(self) -> Packet {
        self.0
    }
}

impl AsRef<[u8]> for OutPacket {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
