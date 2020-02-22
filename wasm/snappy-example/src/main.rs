#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, stdio};

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let fout = stdio::out();
    let mut fin = stdio::inp();

    let mut writer = snap::write::FrameEncoder::new(fout);
    std::io::copy(&mut fin, &mut writer).unwrap();

    Ok(())
}
