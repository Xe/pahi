extern crate httparse;
extern crate olin;

use olin::Resource;
use olin::*;
use std::io::{Read, Write, Cursor};

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
    fout.write("\r\n".as_bytes()).unwrap();

    let mut response: Vec<u8> = vec![];
    fout.read(&mut response).map_err(|why| {
        log::error(&format!("can't read response: {}", why));
        1
    })?;

    let resp = maj::Response::parse(&mut Cursor::new(response)).map_err(|why| {
        log::error(&format!("can't parse gemini response: {}", why));
        1
    })?;

    log::info(&format!("{:?}", resp.status));
    log::info(&format!("{}", resp.meta));
    log::info(&format!("{}", std::str::from_utf8(&resp.body).unwrap()));

    Ok(())
}
