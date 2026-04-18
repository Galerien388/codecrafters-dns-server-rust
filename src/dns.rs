use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

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
    pub fn new_reponse(id: u16) -> Self {
        Self {
            id,
            qr: true,
            ..Default::default()
        }
    }

    pub fn from_bytes() {
        unimplemented!()
    }

    pub fn flags_as_u16(&self) -> u16 {
        let mut flags: u16 = 0;

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

    pub fn write_to(&self, buf: &mut [u8]) {
        BigEndian::write_u16(&mut buf[..2], self.id);
        BigEndian::write_u16(&mut buf[2..4], self.flags_as_u16());
        BigEndian::write_u16(&mut buf[4..6], self.qdcount);
        BigEndian::write_u16(&mut buf[6..8], self.ancount);
        BigEndian::write_u16(&mut buf[8..10], self.nscount);
        BigEndian::write_u16(&mut buf[10..12], self.arcount);
    }
}

pub struct Question {
    pub name: String,
    pub q_type: u16,
    pub q_class: u16,
}

impl Question {
    pub fn new(name: String, q_type: u16, q_class: u16) -> Self {
        Self {
            q_type,
            q_class,
            name,
        }
    }

    pub fn read_question(&self, buf: &mut [u8]) -> usize {
        let names = self.name.split(".");
        let mut start = 0;
        for name in names {
            let len_as_u8 = name.len() as u8;
            buf[start] = len_as_u8;
            start += 1;
            for b in name.as_bytes() {
                buf[start] = *b;
                start += 1;
            }
        }
        buf[start] = b'\0';
        start += 1;
        buf[start..start + 2].copy_from_slice(&self.q_type.to_be_bytes());
        start += 2;
        buf[start..start + 2].copy_from_slice(&self.q_class.to_be_bytes());
        start += 2;
        start
    }
}

pub struct Message {
    header: DnsHeader,
    questions: Vec<Question>,
}

impl Message {
    pub fn new(id: u16) -> Self {
        Self {
            header: DnsHeader::new_reponse(id),
            questions: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.header.qdcount = self.questions.len() as u16;
    }

    pub fn write_header(&self, buf: &mut [u8]) {
        self.header.write_to(buf);
    }

    pub fn write_questions(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for question in &self.questions {
            let len = question.read_question(&mut buf[start..]);
            start += len;
        }
        start
    }
}

// pub struct DnsMessage {
//     header: DnsHeader,
//     questions: Vec<Question>,
//     answers: Vec<Record>,
//     authorities: Vec<Record>,
//     additionals: Vec<Record>,
// }
