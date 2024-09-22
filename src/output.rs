#![allow(non_camel_case_types)]

use piston_window::*;
use piston_window::graphics::math::Scalar;
use std::{net::UdpSocket, sync::mpsc::Receiver, time::Duration};

use crate::utils::{compare, offset};

pub fn piston(rx: Receiver<Vec<[f32; 4]>>) {
    let mut window: PistonWindow = 
        WindowSettings::new("ELApse", [800, 800])
        .transparent(true)
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        let r = match rx.recv_timeout(Duration::from_millis(66)) {
            Ok(e) => e,
            Err(_) => vec![[0.02, 0.02, 0.02, 1.0]; 480],
        };

        let size = window.size();
        let wh = compare(size.height, size.width)*0.75;
        window.draw_2d(&e, |c, g, _device| {
            for e in 1..9 {
                for i in 0..60 {
                    let rwh = wh*(1.0-(e as f64*0.06));
                    circle_arc(r[i*8+(e-1)], rwh/32.0, Scalar::deg_to_rad(offset((((360/60)*i)) as i16, 360, 90) as f64), Scalar::deg_to_rad(offset((((360/60)*i)+6) as i16, 360, 90) as f64), [(size.width/2.0)-(rwh/2.0), (size.height/2.0)-(rwh/2.0), rwh, rwh], c.transform, g);
                }
            }
        });
    }
}

pub fn realtimeudp(rx: Receiver<Vec<[f32; 4]>>, addr: String) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    loop {
        let leds = rx.recv().unwrap();
        let mut packet: Vec<u8> = vec![2, 2];
        for led in leds {
            packet.push((led[0]*255.0) as u8);
            packet.push((led[1]*255.0) as u8);
            packet.push((led[2]*255.0) as u8);
        }
        match socket.send_to(&packet, &addr) {
            Ok(_) => (),
            Err(err) => {
                println!("Error while sending trying to send data: {}", err);
                std::process::exit(1);
            },
        };
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Output {
    piston,
    realtimeudp,
}