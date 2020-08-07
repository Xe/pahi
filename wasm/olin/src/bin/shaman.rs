#![no_main]
#![feature(start)]

use log::error;
use olin::{entrypoint, stdio};
use std::io::Write;

olin::entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let bytes = include_bytes!("shaman.aa");
    let mut out = stdio::out();

    out.write(bytes)
        .map_err(|e| {
            error!("can't write to stdout: {:?}", e);
            1
        })
        .unwrap();

    Ok(())
}
