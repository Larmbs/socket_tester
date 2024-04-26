use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}};

fn main() {
    let address: SocketAddr = "127.0.0.1:5555".parse().unwrap();
    let mut stream = TcpStream::connect(address).unwrap();

    let items = vec!["Hi there", "this is a bot", "who can send messages"];

    for item in items {
        stream.write(item.as_bytes()).unwrap();

        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();

        let string = String::from_utf8_lossy(&buf);
        println!("Sent {}: Received {}", item, string);
    }

}