#![no_main]
#![feature(start)]

extern crate olin;

use log::{error, info, warn};
use olin::entrypoint;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let string = "hi";
    error!("{}", string);
    warn!("{}", string);
    info!("{}", string);

    Ok(())
}
