#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, log, startup};

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let argc = startup::arg_len();
    log::info(&format!("argc: {}", argc));

    Ok(())
}
