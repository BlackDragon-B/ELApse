#![feature(array_chunks)]
mod input;
mod output;
mod parser;
mod utils;
mod fileparser;

use std::sync::mpsc::{self, Receiver, Sender};
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input to use
    #[arg(short, long)]
    input: String,

    /// Output to use
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let (tx, rx): (Sender<Vec<[f32; 4]>>, Receiver<Vec<[f32; 4]>>) = mpsc::channel();
    match args.input.as_str() {
        "-" | "stdin" => input::stdin(tx),
        _ => {
            match Url::parse(&args.input) {
                Ok(o) => {
                    match o.scheme() {
                        "udp" => {
                            input::realtimeudp(tx, utils::get_host(o));
                        },
                        "file" => input::file(tx, o.path().to_owned()),
                        _ => {
                            println!("Unrecognized uri scheme");
                            std::process::exit(1);
                        }
                    }
                },
                Err(_) => {
                    input::file(tx, args.input);
                },
            };
        }
    }
    match args.output.as_str() {
        "piston" => output::piston(rx),
        _ => {
            match Url::parse(&args.output) {
                Ok(o) => {
                    match o.scheme() {
                        "udp" => output::realtimeudp(rx,utils::get_host(o)),
                        _ => {
                            println!("Unrecognized uri scheme");
                            std::process::exit(1);
                        }
                    }
                },
                Err(_) => {
                    println!("Output out of scope");
                    std::process::exit(1);
                },
            };
        }
    }
}
