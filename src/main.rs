#[allow(unused_imports, dead_code, unused)]
use std::net::UdpSocket;

use bytes::buf;

use crate::dns::{Message, Question};

pub mod dns;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    //
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut response = [0; 512];
                let mut msg = Message::new(1234);
                msg.add_question(Question {
                    name: "codecrafters.io".to_string(),
                    q_type: 1 as u16,
                    q_class: 1 as u16,
                });

                msg.write_header(&mut response);
                msg.write_questions(&mut response);

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
