#![no_main]
#![feature(start)]

extern crate olin;

use log::info;
use olin::{entrypoint, startup};

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let argc = startup::arg_len();
    info!("argc: {}", argc);

    Ok(())
}
