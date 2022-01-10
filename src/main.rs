use std::net::UdpSocket;
use std::str;
use std::env;
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

fn usage() {
    eprintln!("usage: hsl <address:port> <filename|->");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage();
        return Ok(());
    }

    let address = &args[1];
    let filename = &args[2];
    let output = match filename {
        _ if filename == "-" => "STDOUT",
        _ => filename
    };

    let mut file: Option<File> = None;
    if !filename.eq("-") {
        file = Some(File::create(filename).unwrap());
    }

    let socket = UdpSocket::bind(address)?;
    println!("Running server on address {} with output to {}", address, output);

    let mut buf = [0; 8 * 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        let str = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e)
        };
        let now = Utc::now();
        match file {
            None => print!("{} client[{}] - {}", now.to_string(), src, str),
            _ => write!(file.as_ref().unwrap(), "{} client[{}] - {}", now.to_string(), src, str)?
        }
    }
}