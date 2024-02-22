use std::net::UdpSocket;

const SERVER: &str = "127.0.0.1:20002";

pub fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind("0.0.0.0:0")
}

pub fn send_buffer_key(socket: &UdpSocket, buffer: &Vec<rdev::Key>) {
    let key_str = buffer.iter()
        .map(|k| format!("{:?}", k))
        .collect::<Vec<String>>()
        .join(" ");

    socket.send_to(key_str.as_bytes(), SERVER);
}