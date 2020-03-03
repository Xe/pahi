#![no_main]
#![feature(start)]

extern crate olin;

use olin::entrypoint;
use serde_json::{from_slice, to_string, Value};

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let input = include_bytes!("./bigjson.json");

    if let Ok(val) = from_slice(input) {
        let v: Value = val;
        if let Err(_why) = to_string(&v) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "oh no json encoding failed!",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no json parsing failed!",
        ));
    }

    Ok(())
}
