#![feature(array_chunks)]
mod input;
mod output;
mod parser;
mod utils;
mod fileparser;

use std::sync::mpsc::{self, Receiver, Sender};
use clap::{command, error::{self, ContextKind, ContextValue, ErrorKind}, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input driver to use
    #[arg(short, long)]
    input: input::Input,

    /// File input for the file driver
    #[arg(required(false), short, long, requires = "input")]
    file: Option<String>,

    /// Output driver to use
    #[arg(short, long)]
    output: output::Output,

    /// Destination IP for the realtimeudp driver
    #[arg(required(false), short, long, requires = "output")]
    destip: Option<String>,
}

fn main() {
    let args = Args::parse();
    // let mut arg = clap::command!()
    //     .arg(clap::arg!(-o --output <file> "The output file to write to. Use - for stdout."))
    //     .arg_required_else_help(true);
    let (tx, rx): (Sender<Vec<[f32; 4]>>, Receiver<Vec<[f32; 4]>>) = mpsc::channel();
    // if x {
    //     input::stdin(tx);
    // } else {
    //     input::realtimeudp(tx)
    // }
    match args.input {
        input::Input::stdin => input::stdin(tx),
        input::Input::realtimeudp => input::realtimeudp(tx),
        input::Input::file => {
            if args.file.is_some() {
                input::file(tx, args.file.unwrap());
            } else {
                utils::err_missing_args(vec!["--file".to_owned()])
            }
        },
    }
    // input::file(tx);
    // output::piston(rx) 21324
    match args.output {
        output::Output::piston => output::piston(rx),
        output::Output::realtimeudp => {
            if args.destip.is_some() {
                output::realtimeudp(rx, args.destip.unwrap())
            } else {
                utils::err_missing_args(vec!["--destip".to_owned()])
            }
        },
    }
}
