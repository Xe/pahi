#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, stdio};
use std::io::Write;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let fout = stdio::out();

    let mut writer = snap::write::FrameEncoder::new(fout);
    writer.write(include_bytes!("/proc/cpuinfo"))?;

    Ok(())
}
