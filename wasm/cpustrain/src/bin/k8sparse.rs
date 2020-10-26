#![no_main]
#![feature(start)]

extern crate olin;

use log::*;
use olin::{entrypoint, Resource};
use serde_yaml::{from_slice, to_string, Value};
use std::io::Write;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let input = include_bytes!("./k8sparse.yaml");
    let mut res = Resource::open("null://").expect("open null://");

    match from_slice(input) {
        Ok(val) => {
            let v: Value = val;

            match to_string(&v) {
                Err(why) => {
                    error!("yaml encoding failed: {}", why);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "oh no yaml encoding failed!",
                    ));
                }
                Ok(val) => {
                    res.write(val.as_bytes())
                        .expect("able to write yaml to dev null");
                }
            };
        }
        Err(why) => panic!("{}", why),
    };

    Ok(())
}
