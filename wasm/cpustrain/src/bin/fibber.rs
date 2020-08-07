#![no_main]
#![feature(start)]

extern crate olin;

use log::info;
use olin::entrypoint;

entrypoint!();

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() -> Result<(), std::io::Error> {
    info!("starting");
    fib(30);
    info!("done");
    Ok(())
}
