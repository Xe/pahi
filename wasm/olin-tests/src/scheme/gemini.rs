extern crate httparse;
extern crate olin;

use olin::Resource;
use olin::*;
use std::io::{Read, Write};

pub extern "C" fn test() -> Result<(), i32> {
    log::info("running scheme::gemini tests");
    const request: &'static str = "gemini://cetacean.club/";

    let mut fout: Resource = Resource::open(request).map_err(|why| {
        log::error(&format!("couldn't open {}: {}", request, why));
        1
    })?;

    fout.write(request.as_bytes()).map_err(|why| {
        log::error(&format!("can't write request: {}", why));
        1
    })?;

    let mut response = [0u8; 4096];
    fout.read(&mut response).map_err(|why| {
        log::error(&format!("can't read response: {}", why));
        1
    })?;

    let resp = maj::Response::parse(&mut response).map_err(|why| {
        log::error(&format!("can't parse gemini response: {}", why));
        1
    })?;

    Ok(())
}
