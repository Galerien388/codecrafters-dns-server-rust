#[allow(unused_imports, dead_code, unused)]
use std::net::UdpSocket;
use std::{env, ops::Deref};

use bytes::buf;

use crate::dns::{Answer, Message, Question};

pub mod dns;

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

                let mut msg = Message::from_header_bytes(&mut buf[..FLAG_SIZE]);

                let _len = msg.read_questions(&mut buf[FLAG_SIZE..]);

                let (mut resp_msg, _size) = match resolver_addr {
                    Some(ref resolver) => query_msg(msg, resolver.as_str(), &udp_socket),
                    None => {
                        let mut m = Message::new(msg.header.id);
                        for q in msg.questions {
                            m.answers.push(Answer::new(
                                q.name,
                                q.q_type,
                                q.q_class,
                                60,
                                4,
                                "8.8.8.8".to_string(),
                            ));
                        }
                        (m, 0)
                    }
                };

                let mut response = [0; 512];

                let mut len = resp_msg.write_questions(&mut response[..FLAG_SIZE]);
                len += resp_msg.write_answers(&mut response[len..]);

                //
                //
                //
                //
                //
                //
                //
                // msg.write_header(&mut response);
                // let mut len = FLAG_SIZE;
                // len += msg.write_questions(&mut response[len..]);
                // len += msg.write_answers(&mut response[len..]);

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

fn query_msg(message: Message, resolver_addr: &str, udp_socket: &UdpSocket) -> (Message, usize) {
    let mut start = 0;

    let mut msg_response = Message::new(message.header.id);

    for question in message.questions {
        let mut msg = Message::new(message.header.id);
        msg.add_question(question);
        let mut req = [0; 512];
        let len = write_questions(msg, &mut req);

        udp_socket
            .send_to(&req[..len], resolver_addr)
            .expect("Failed to send req to resolver");

        let mut resp_buf = [0; 512];
        let (size, _addr) = udp_socket
            .recv_from(&mut resp_buf)
            .expect("Failed to receive from resolver");

        let mut msg_received = Message::from_header_bytes(&resp_buf[..FLAG_SIZE]);
        let mut len = FLAG_SIZE;
        len += msg_received.read_questions(&resp_buf[len..size]);
        msg_received.read_answers(&resp_buf[len..size]);

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
    message.write_header(&mut buf[..FLAG_SIZE]);
    let len = message.write_questions(&mut buf[FLAG_SIZE..]);
    len + FLAG_SIZE
}
