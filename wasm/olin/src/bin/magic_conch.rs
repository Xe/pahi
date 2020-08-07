#![no_main]
#![feature(start)]
#![macro_use]
extern crate olin;

use log::info;
use olin::{entrypoint, env};

entrypoint!();

fn main() -> Result<(), env::Error> {
    let name = "MAGIC_CONCH";
    let value = env::get(name)?;

    info!("{} -> {}", name, value);

    Ok(())
}
