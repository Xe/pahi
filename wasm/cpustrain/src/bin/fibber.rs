#![no_main]
#![feature(start)]

extern crate olin;

use olin::{entrypoint, log};

entrypoint!();

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() -> Result<(), std::io::Error> {
    log::info("starting");
    fib(30);
    log::info("done");
    Ok(())
}
