// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::io::Read;
use std::path::PathBuf;
use std::{env, fs::File};
use vita49::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("error - pass a JSON5 file");
    }

    let mut path = PathBuf::from(&args[1]);

    let mut input = match std::fs::File::open(&path) {
        Ok(file) => Box::new(file),
        Err(err) => {
            panic!("{}: {}", path.to_string_lossy(), err);
        }
    };

    // Read the passed file into a string.
    let mut json = String::new();
    input
        .read_to_string(&mut json)
        .expect("failed to read string from file");

    // Deserialize the JSON string to a VRT packet.
    let packet: Vrt = serde_json5::from_str(&json).expect("failed to parse JSON into VRT");

    // Write the raw packet data to the same file name, but with ".vrt" extension.
    let _ = path.set_extension("vrt");
    let file = File::options()
        .write(true)
        .create(true)
        .open(&path)
        .expect("failed to open VRT file");
    packet
        .to_writer(&mut Writer::new(file), ())
        .expect("failed to write VRT file");

    println!("Wrote VRT data to {}", path.to_string_lossy());
}
