#![allow(non_camel_case_types)]
use std::{io::Read, net::UdpSocket, sync::mpsc::{self, Receiver, Sender}, thread, time::{Duration, Instant}};
use crate::{ parser, fileparser };

pub fn stdin(tx: Sender<Vec<[f32; 4]>>) {
    let (stdintx, stdinrx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    thread::spawn( move || {
        let mut buf = [0; 1500];
        let mut e = std::io::stdin().lock();
        loop {
            let amt = e.read(&mut buf).unwrap();
            if buf[..amt].to_vec().len() > 0 {
                let _ = stdintx.send(buf[..amt].to_vec());
            }
        }
    });
    thread::spawn( move || {
        loop {
            // let amt = e.read(&mut buf).unwrap();
            match stdinrx.recv_timeout(Duration::from_millis(500)) {
                Ok(d) => {
                    let r: Vec<[f32; 4]> = parser::parseleds(d, parser::ParserOptions::default().mirror(true).double(true).offset(15));
                    let _ = tx.send(r);
                },
                Err(_) => {
                    std::process::exit(0);
                },
            };
        }
    });
}

pub fn realtimeudp(tx: Sender<Vec<[f32; 4]>>,bindaddr: String) {
    thread::spawn( move || {
        let socket = UdpSocket::bind(bindaddr).unwrap();
        let mut buf = [0; 1500];
        loop {
            let (amt, _src) = socket.recv_from(&mut buf).unwrap();
            let r = parser::parseleds(buf[..amt].to_vec(), parser::ParserOptions::default().stripheader(2));
            let _ = tx.send(r);    
        }
    });
}

pub fn file(tx: Sender<Vec<[f32; 4]>>, path: String) {
    thread::spawn( move || {
        let Ok(rawbytes) = fileparser::parse(&path) else {
            println!("File not found");
            std::process::exit(1)
        };
        for frame in rawbytes.chunks(720) {
            let st = Instant::now();
            let r = parser::parseleds(frame.to_vec(), parser::ParserOptions::default().mirror(true).double(true).offset(15));
            let _ = tx.send(r);
            if Duration::from_secs_f32(1.0/60.0) > st.elapsed() {
                thread::sleep(Duration::from_secs_f32(1.0/60.0)-st.elapsed());
            };
        }
        std::process::exit(0);
    });
}
