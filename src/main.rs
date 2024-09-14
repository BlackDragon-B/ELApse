#![feature(array_chunks)]
mod input;
mod output;
mod parser;
mod utils;
mod fileparser;

use std::sync::mpsc::{self, Receiver, Sender};
use clap::{command, Parser};

fn main() {
    // let mut arg = clap::command!()
    //     .arg(clap::arg!(-o --output <file> "The output file to write to. Use - for stdout."))
    //     .arg_required_else_help(true);
    let x = true;
    let (tx, rx): (Sender<Vec<[f32; 4]>>, Receiver<Vec<[f32; 4]>>) = mpsc::channel();
    // if x {
    //     input::stdin(tx);
    // } else {
    //     input::realtimeudp(tx)
    // }
    input::file(tx);
    output::realtimeudp(rx)
 

}
