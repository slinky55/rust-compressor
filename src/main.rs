use std::env;

use rust_compressor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => compress(&args[1], &args[2]),
        4 => {
            let options = &args[3..];

            for option in options {
                match option.as_str() {
                    "-c" => compress(&args[1], &args[2]),
                    "-d" => decompress(&args[1], &args[2]),
                    _ => {
                        eprintln!("Unknown option: {}", option);
                        std::process::exit(1);
                    }
                }
            }
        }
        _ => {
            eprintln!("Usage: {} <src> <dst> [options...]", args[0]);
            std::process::exit(1);
        }
    }
}


