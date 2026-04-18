#[allow(unused_imports, dead_code, unused)]
use std::net::UdpSocket;

use bytes::buf;

use crate::dns::{Answer, Message, Question};

pub mod dns;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    //
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut msg = Message::from_request(&mut buf);

                let mut response = [0; 512];

                // let question = Question {
                //     name: "codecrafters.io".to_string(),
                //     q_type: 1 as u16,
                //     q_class: 1 as u16,
                // };
                //
                // msg.add_question(question);
                let _len = msg.read_questions(&mut buf[12..]);
                msg.add_answer(Answer::new(
                    "codecrafters.io".to_string(),
                    1,
                    1,
                    60,
                    4,
                    "8.8.8.8".to_string(),
                ));

                msg.write_header(&mut response);
                let mut len = 12;
                len += msg.write_questions(&mut response[len..]);
                len += msg.write_answers(&mut response[len..]);

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
