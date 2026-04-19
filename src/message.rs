use crate::{
    answer::Answer,
    header::{DnsHeader, HEADER_LEN},
    question::Question,
};

#[derive(Debug)]
pub struct Message {
    pub header: DnsHeader,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn new(id: u16) -> Self {
        Self {
            header: DnsHeader::new_reponse(id),
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn set_request(&mut self) {
        self.header.flags.set_req();
    }

    pub fn set_reply(&mut self) {
        self.header.flags.set_resp();
    }

    pub fn header_from_slice(buf: &[u8]) -> (Self, usize) {
        let mut header = DnsHeader::from_bytes(buf);
        header.flags.rcode = if header.flags.opcode == 0 { 0 } else { 4 };
        (
            Self {
                header: header,
                questions: Vec::new(),
                answers: Vec::new(),
            },
            HEADER_LEN,
        )
    }

    pub fn questions_from_slice(&mut self, buf: &[u8]) -> usize {
        let mut start = 0;
        for _ in 0..self.header.qdcount {
            let (question, len) = Question::from_slice(&buf[start..]);
            println!("Question read: {:?}", question);
            start += len;
            self.add_question(question);
        }
        start
    }

    pub fn answers_from_slice(&mut self, buf: &[u8]) -> usize {
        let mut start = 0;
        for _ in 0..self.header.ancount {
            let (answer, len) = Answer::from_slice(&buf[start..]);
            println!("Answer read: {:?}", answer);
            start += len;
            self.add_answer(answer);
        }
        start
    }

    pub fn add_question(&mut self, question: Question) {
        println!("push question: {:?}", question);
        self.questions.push(question);
        self.header.qdcount = self.questions.len() as u16;
    }

    pub fn add_answer(&mut self, answer: Answer) {
        println!("push answer: {:?}", answer);
        self.answers.push(answer);
        self.header.ancount = self.answers.len() as u16;
    }

    pub fn header_into_slice(&self, buf: &mut [u8]) -> usize {
        self.header.into_bytes(buf);
        HEADER_LEN
    }

    pub fn questions_into_slice(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for question in &self.questions {
            let len = question.into_slice(&mut buf[start..]);
            start += len;
        }
        start
    }

    pub fn answers_into_slice(&self, buf: &mut [u8]) -> usize {
        let mut start = 0;
        for answer in &self.answers {
            let len = answer.into_slice(&mut buf[start..]);
            start += len;
        }
        start
    }
}
