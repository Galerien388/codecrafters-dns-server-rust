use anyhow::{Context, Result};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io::Write;

// [ ID        ] 2 bytes
// [ FLAGS     ] 2 bytes
// [ QDCOUNT   ] 2 bytes
// [ ANCOUNT   ] 2 bytes
// [ NSCOUNT   ] 2 bytes
// [ ARCOUNT   ] 2 bytes

#[derive(Debug, Default)]
pub struct DnsHeader {
    id: u16,
    //Query/Response Indicator
    qr: bool,
    opcode: u8,
    //Authoritative Answer
    aa: bool,
    // Truncation
    tc: bool,
    // Recursion desired
    rd: bool,
    // Recursion available
    ra: bool,
    // resevered
    z: u8,
    // Response code
    rcode: u8,
    // Question count
    qdcount: u16,
    // Answer record count
    ancount: u16,
    // Authoritie record count
    nscount: u16,
    // Additional record count
    arcount: u16,
}

impl DnsHeader {
    pub fn new_reponse(_id: u16) -> Self {
        Self {
            id: 1234,
            qr: true,
            ..Default::default()
        }
    }

    pub fn from_bytes() {
        unimplemented!()
    }

    pub fn flags_as_u16(&self) -> u16 {
        let mut flags: u16 = 0;
        // | OR
        flags |= (self.qr as u16) << 15;
        flags |= (self.opcode as u16) << 11;
        flags |= (self.aa as u16) << 10;
        flags |= (self.tc as u16) << 9;
        flags |= (self.rd as u16) << 8;
        flags |= (self.ra as u16) << 7;
        flags |= (self.z as u16) << 4;
        flags |= self.rcode as u16;
        flags
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buff = [0u8; 12];
        BigEndian::write_u16(&mut buff[..2], self.id);
        BigEndian::write_u16(&mut buff[2..4], self.flags_as_u16());
        BigEndian::write_u16(&mut buff[4..6], self.qdcount);
        BigEndian::write_u16(&mut buff[6..8], self.ancount);
        BigEndian::write_u16(&mut buff[8..10], self.nscount);
        BigEndian::write_u16(&mut buff[10..], self.arcount);
        buff
    }
}

// pub struct DnsMessage {
//     header: DnsHeader,
//     questions: Vec<Question>,
//     answers: Vec<Record>,
//     authorities: Vec<Record>,
//     additionals: Vec<Record>,
// }
