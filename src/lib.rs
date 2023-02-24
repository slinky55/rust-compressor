use std::io::{Read, Write};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

pub fn compress(src: &str, dst: &str) {
    let contents = std::fs::read(src).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    });

    let mut encoder =
        GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&contents).unwrap_or_else(|err| {
        eprintln!("Problem compressing file: {}", err);
        std::process::exit(1);
    });

    let compressed_bytes = encoder.finish().unwrap_or_else(|err| {
        eprintln!("Problem finishing compression: {}", err);
        std::process::exit(1);
    });

    std::fs::write(dst, compressed_bytes).unwrap_or_else(|err| {
        eprintln!("Problem writing file: {}", err);
        std::process::exit(1);
    });
}

pub fn decompress(src: &str, dst: &str) {
    let contents = std::fs::read(src).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    });

    let mut decoder = GzDecoder::new(&contents[..]);
    let mut decompressed_bytes = Vec::new();
    decoder.read_to_end(&mut decompressed_bytes).unwrap_or_else(|err| {
        eprintln!("Problem decompressing file: {}", err);
        std::process::exit(1);
    });

    std::fs::write(dst, decompressed_bytes).unwrap_or_else(|err| {
        eprintln!("Problem writing file: {}", err);
        std::process::exit(1);
    });
}