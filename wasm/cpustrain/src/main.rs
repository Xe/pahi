#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, Resource};
use std::io::Write;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let fout = Resource::open("null://").expect("opening /dev/null");
    let data = include_bytes!("/proc/cpuinfo");

    let mut writer = snap::write::FrameEncoder::new(fout);

    for _ in 0..256 {
        // compressed data
        writer.write(data)?;
    }

    Ok(())
}
