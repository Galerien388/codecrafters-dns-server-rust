use std::env;
#[allow(unused_imports, dead_code, unused)]
use std::net::UdpSocket;

use crate::{answer::Answer, header::HEADER_LEN, message::Message};

pub mod answer;
pub mod field;
pub mod header;
pub mod message;
pub mod question;

const FLAG_SIZE: usize = 12;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let resolver_addr = env::args().nth(2);
    let mut buf = [0; 512];

    //
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let (mut req_msg, req_len) = Message::header_from_slice(&buf[..HEADER_LEN]);
                req_msg.questions_from_slice(&buf[req_len..size]);

                let (mut resp_msg, _size) = match resolver_addr {
                    Some(ref resolver) => {
                        println!("resolver is set");
                        query_msg(req_msg, resolver.as_str())
                    }
                    None => {
                        println!("resolver is not set");
                        let mut m = Message::new(req_msg.header.id);
                        for q in req_msg.questions {
                            m.answers.push(Answer::new(
                                q.field.name.clone(),
                                q.field.f_type,
                                q.field.f_class,
                                60,
                                4,
                                q.field.name.clone(),
                            ));
                            m.questions.push(q);
                        }
                        (m, 0)
                    }
                };

                let mut response = [0; 512];

                resp_msg.header.flags.set_resp();
                let mut len = resp_msg.header_into_slice(&mut response[..FLAG_SIZE]);
                len += resp_msg.questions_into_slice(&mut response[len..size]);

                udp_socket
                    .send_to(&response[..len], source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn query_msg(message: Message, resolver_addr: &str) -> (Message, usize) {
    let mut start = 0;

    let mut msg_response = Message::new(message.header.id);

    for question in message.questions {
        println!("Send msg to resolver with question: {:?}", question);
        let mut msg = Message::new(message.header.id);
        msg.add_question(question);
        let mut req = [0; 512];
        let len = write_questions(msg, &mut req);

        let udp_socket = UdpSocket::bind("127.0.0.1:2055").expect("Failed to bind to address");
        udp_socket
            .send_to(&req[..len], resolver_addr)
            .expect("Failed to send req to resolver");

        let mut resp_buf = [0; 512];

        let (size, _addr) = udp_socket
            .recv_from(&mut resp_buf)
            .expect("Failed to receive from resolver");

        let (mut msg_received, mut len) = Message::header_from_slice(&resp_buf[..HEADER_LEN]);
        len += msg_received.questions_from_slice(&resp_buf[len..size]);
        msg_received.answers_from_slice(&resp_buf[len..size]);

        for q in msg_received.questions {
            msg_response.questions.push(q);
        }

        for a in msg_received.answers {
            msg_response.answers.push(a);
        }

        start += len;
    }

    (msg_response, start)
}

fn write_questions(message: Message, buf: &mut [u8]) -> usize {
    let mut start = message.header_into_slice(&mut buf[..HEADER_LEN]);
    start += message.questions_into_slice(&mut buf[start..]);
    start
}
