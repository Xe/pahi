#![no_main]
#![feature(start)]

extern crate olin;

use blake2::{Blake2b, Digest};
use olin::{entrypoint, log};

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let json: &'static [u8] = include_bytes!("./bigjson.json");
    let yaml: &'static [u8] = include_bytes!("./k8sparse.yaml");
    for _ in 0..8 {
        let mut hasher = Blake2b::new();
        hasher.input(json);
        hasher.input(yaml);
        hasher.result();
    }

    Ok(())
}
