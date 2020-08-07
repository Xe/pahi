#![allow(unused_must_use)]
#![no_main]
#![feature(start)]

mod ns;
mod olintest;
mod regression;
mod scheme;

use log::error;
use olin::{entrypoint, runtime::exit};
use std::io::Error;

entrypoint!();

fn main() -> Result<(), std::io::Error> {
    let mut fail_count = 0;

    let funcs = [
        ns::env::test,
        ns::random::test,
        ns::resource::test,
        ns::runtime::test,
        ns::startup::test,
        ns::stdio::test,
        ns::time::test,
        olintest::http::test,
        regression::issue22::test,
        regression::issue37::test,
        regression::issue39::test,
        //scheme::gemini::test,
        scheme::http::test,
        scheme::log::test,
        scheme::null::test,
        scheme::random::test,
        scheme::zero::test,
    ];

    for func in &funcs {
        match func() {
            Ok(()) => {}
            Err(e) => {
                error!("test error: {:?}", e);
                fail_count += 1;
            }
        }
    }

    exit(fail_count);
}
