use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::buf;

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

    pub fn from_bytes(buf: &[u8]) -> Self {
        let flags = u16::from_be_bytes([buf[2], buf[3]]);

        let (qr, opcode, aa, tc, rd, ra, z, rcode) = Self::u16_to_flags(flags);

        Self {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
        }
    }

    pub fn u16_to_flags(flags: u16) -> (bool, u8, bool, bool, bool, bool, u8, u8) {
        const OPCODE_MASK: u16 = 0b1111;
        const Z_MASK: u16 = 0b111;
        const RCODE_MASK: u16 = 0b1111;
        (
            (flags >> 15) & 1 == 1,
            ((flags >> 11) & OPCODE_MASK) as u8,
            (flags >> 10) & 1 == 1,
            (flags >> 9) & 1 == 1,
            (flags >> 8) & 1 == 1,
            (flags >> 7) & 1 == 1,
            ((flags >> 4) & Z_MASK) as u8,
            ((flags) & RCODE_MASK) as u8,
        )
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

    pub fn write_to(&self, buf: &mut [u8]) -> usize {
        encode_name(buf, &self.name, self.q_type, self.q_class)
    }

    pub fn read_from(buf: &[u8]) -> (Question, usize) {
        let mut start = 0;
        let mut names = Vec::new();

        loop {
            let len = buf[start] as usize;
            start += 1;
            if len == 0 {
                break;
            }
            let word = str::from_utf8(&buf[start..start + len]).expect("invalid word");
            names.push(word);
            start += len;
        }

        let q_type = u16::from_be_bytes([buf[start], buf[start + 1]]);
        start += 2;
        let q_class = u16::from_be_bytes([buf[start], buf[start + 1]]);
        start += 2;

        (
            Question {
                name: names.join(".").to_string(),
                q_type,
                q_class,
            },
            start,
        )
    }
}

fn encode_name(buf: &mut [u8], name: &str, name_type: u16, name_class: u16) -> usize {
    let names = name.split('.');
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
    buf[start..start + 2].copy_from_slice(&name_type.to_be_bytes());
    start += 2;
    buf[start..start + 2].copy_from_slice(&name_class.to_be_bytes());
    start += 2;
    start
}

pub struct Answer {
    pub name: String,
    pub a_type: u16,
    pub a_class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: u32,
}

impl Answer {
    pub fn new(name: String, a_type: u16, a_class: u16, ttl: u32, length: u16, ip: String) -> Self {
        Self {
            name,
            a_type,
            a_class,
            ttl,
            data: Self::ip_to_u32(ip),
            length,
        }
    }

    fn ip_to_u32(ip: String) -> u32 {
        let mut octets = [0u8; 4];
        for (i, part) in ip.split('.').enumerate() {
            octets[i] = part.parse::<u8>().expect("invalid IP");
        }
        u32::from_be_bytes(octets)
    }

    pub fn write_to(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        let q_len = encode_name(buf, &self.name, self.a_type, self.a_class);
        start += q_len;
        buf[start..start + 4].copy_from_slice(&self.ttl.to_be_bytes());
        start += 4;
        buf[start..start + 2].copy_from_slice(&self.length.to_be_bytes());
        start += 2;
        buf[start..start + 4].copy_from_slice(&self.data.to_be_bytes());
        start += 4;
        start
    }
}

pub struct Message {
    header: DnsHeader,
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl Message {
    pub fn new(id: u16) -> Self {
        Self {
            header: DnsHeader::new_reponse(id),
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn from_request(buf: &mut [u8]) -> Self {
        let mut header = DnsHeader::from_bytes(buf);
        header.qr = true;
        header.rcode = if header.opcode == 0 { 0 } else { 4 };

        Self {
            header: header,
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn read_questions(&mut self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for _ in 0..self.header.qdcount {
            let (question, len) = Question::read_from(&mut buf[start..]);
            start += len;
            self.questions.push(question);
        }
        start
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.header.qdcount = self.questions.len() as u16;
    }

    pub fn add_answer(&mut self, answer: Answer) {
        self.answers.push(answer);
        self.header.ancount = self.answers.len() as u16;
    }

    pub fn write_header(&self, buf: &mut [u8]) {
        self.header.write_to(buf);
    }

    pub fn write_questions(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for question in &self.questions {
            let len = question.write_to(&mut buf[start..]);
            start += len;
        }
        start
    }

    pub fn write_answers(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for answer in &self.answers {
            let len = answer.write_to(&mut buf[start..]);
            start += len;
        }
        start
    }
}
