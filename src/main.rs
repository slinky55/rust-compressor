use std::env;

use rust_compressor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 | 2 => {
            eprintln!("Usage: rust-compressor <src> <dst> [-c|-d|-m <method>]");
            std::process::exit(1);
        }
        3 => compress(&args[1], &args[2]),
        _ => {
            let mut options = Vec::from(&args[3..]);

            while options.len() > 0 {
                let option = options.pop().unwrap();
                match option.as_str() {
                    "-c" => compress(&args[1], &args[2]),
                    "-d" => decompress(&args[1], &args[2]),
                    "-m" => {
                        let method = options.pop().unwrap_or_else(|| {
                            eprintln!("Expected method type after -m");
                            std::process::exit(1);
                        });
                    }
                    _ => {
                        eprintln!("Unknown option: {}", option);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}


