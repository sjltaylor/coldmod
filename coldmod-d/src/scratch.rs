use std::net::UdpSocket;
use std::str;

extern crate redis;
use redis::{Client, Commands};

fn scratch() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:23456")?;
    let mut buf = [0; 1024];

    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();

    loop {
        let (amt, _) = socket.recv_from(&mut buf)?;
        let s = str::from_utf8(&buf[..amt]).unwrap();
        let _: () = conn.set("daemon", s).unwrap();
    }
}
