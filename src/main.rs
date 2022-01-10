// use std::fs::File;
use std::net::UdpSocket;
use std::str;
use chrono::prelude::*;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:11111")?;
    println!("Running server....");
    let mut buf = [0; 8 * 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        let str = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e)
        };

        let now = Utc::now();
        print!("{} client[{}] - {}", now.to_string(), src, str)
    }
    // Ok(())
}