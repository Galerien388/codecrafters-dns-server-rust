use byteorder::{BigEndian, ByteOrder};
use std::convert::From;

// [ ID        ] 2 bytes
// [ FLAGS     ] 2 bytes
// [ QDCOUNT   ] 2 bytes
// [ ANCOUNT   ] 2 bytes
// [ NSCOUNT   ] 2 bytes
// [ ARCOUNT   ] 2 bytes

pub const HEADER_LEN: usize = 12;

#[derive(Debug, Default)]
pub struct Flags {
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
}

impl Flags {
    pub fn set_req(&mut self) {
        self.qr = false;
    }

    pub fn set_resp(&mut self) {
        self.qr = true;
    }
}

impl From<u16> for Flags {
    fn from(flags: u16) -> Self {
        const OPCODE_MASK: u16 = 0b1111;
        const Z_MASK: u16 = 0b111;
        const RCODE_MASK: u16 = 0b1111;
        Self {
            qr: (flags >> 15) & 1 == 1,
            opcode: ((flags >> 11) & OPCODE_MASK) as u8,
            aa: (flags >> 10) & 1 == 1,
            tc: (flags >> 9) & 1 == 1,
            rd: (flags >> 8) & 1 == 1,
            ra: (flags >> 7) & 1 == 1,
            z: ((flags >> 4) & Z_MASK) as u8,
            rcode: ((flags) & RCODE_MASK) as u8,
        }
    }
}

impl From<&Flags> for u16 {
    fn from(flags: &Flags) -> Self {
        let mut result: u16 = 0;
        result |= (flags.qr as u16) << 15;
        result |= (flags.opcode as u16) << 11;
        result |= (flags.aa as u16) << 10;
        result |= (flags.tc as u16) << 9;
        result |= (flags.rd as u16) << 8;
        result |= (flags.ra as u16) << 7;
        result |= (flags.z as u16) << 4;
        result |= flags.rcode as u16;
        result
    }
}

#[derive(Debug, Default)]
pub struct DnsHeader {
    pub id: u16,
    flags: Flags,
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
    pub fn new_reponse(id: u16) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn write_into_bytes(&self, buf: &mut [u8]) -> usize {
        BigEndian::write_u16(&mut buf[..2], self.id);
        BigEndian::write_u16(&mut buf[2..4], u16::from(&self.flags));
        BigEndian::write_u16(&mut buf[4..6], self.qdcount);
        BigEndian::write_u16(&mut buf[6..8], self.ancount);
        BigEndian::write_u16(&mut buf[8..10], self.nscount);
        BigEndian::write_u16(&mut buf[10..12], self.arcount);
        HEADER_LEN
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        let flags_as_u16 = u16::from_be_bytes([buf[2], buf[3]]);
        let flags = flags_as_u16.into();

        Self {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            flags,
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
        }
    }
}
