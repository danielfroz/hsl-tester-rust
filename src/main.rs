use std::net::UdpSocket;
use std::str;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use chrono::prelude::*;

const MAXBUF: usize = 8 * 1024;

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

    loop {
        let mut buf = [0; MAXBUF];
        let (mut bytes_read, src) = socket.recv_from(&mut buf)?;
        if bytes_read < MAXBUF && buf[bytes_read] != '\n' as u8 {
            buf[bytes_read] = '\n' as u8;
            bytes_read += 1;
        }
        let buf = &mut buf[..bytes_read];
        let str = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence {}", e)
        };
        let now = Utc::now();
        match file {
            None => {
                print!("{} client[{}] - {}", now.to_string(), src, str);
                io::stdout().flush().unwrap();
            },
            _ => {
                write!(file.as_ref().unwrap(), "{} client[{}] - {}", now.to_string(), src, str)?;
                // sync_all();
            }
        }
    }
}