#![no_main]
#![feature(start)]
#![macro_use]
extern crate olin;

use olin::{entrypoint, env, log};

entrypoint!();

fn main() -> Result<(), env::Error> {
    let name = "MAGIC_CONCH";
    let value = env::get(name)?;

    log::info(&format!("{} -> {}", name, value));

    Ok(())
}
