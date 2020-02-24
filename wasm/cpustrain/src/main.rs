#![no_main]
#![feature(start)]

extern crate olin;

use blake2::{Blake2b, Digest};
use olin::{entrypoint, log, Resource};
use std::io::Write;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let fout = Resource::open("null://").expect("opening /dev/null");
    let data = include_bytes!("/proc/cpuinfo");

    let mut writer = snap::write::FrameEncoder::new(fout);

    for _ in 0..256 {
        // compressed data
        writer.write(data)?;

        // blake-2
        let mut hasher = Blake2b::new();
        hasher.input("lol yolo swag for jesus");
        let res = hasher.result();
        log::info(&format!("blake2: {:?}", res));
    }

    Ok(())
}
