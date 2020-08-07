#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, startup};
use log::info;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let argc = startup::arg_len();
    info!("argc: {}", argc);

    Ok(())
}
