extern crate httparse;
extern crate olin;

use log::{error, info};
use olin::Resource;
use std::io::{Cursor, Read, Write};

pub extern "C" fn test() -> Result<(), i32> {
    info!("running scheme::gemini tests");
    const request: &'static str = "gemini://cetacean.club/";

    let mut fout: Resource = Resource::open(request).map_err(|why| {
        error!("couldn't open {}: {}", request, why);
        1
    })?;

    fout.write(request.as_bytes()).map_err(|why| {
        error!("can't write request: {}", why);
        1
    })?;
    fout.write("\r\n".as_bytes()).unwrap();

    let mut response: Vec<u8> = vec![];
    fout.read(&mut response).map_err(|why| {
        error!("can't read response: {}", why);
        1
    })?;

    let resp = maj::Response::parse(&mut Cursor::new(response)).map_err(|why| {
        error!("can't parse gemini response: {}", why);
        1
    })?;

    info!("{:?}", resp.status);
    info!("{}", resp.meta);
    info!("{}", std::str::from_utf8(&resp.body).unwrap());

    Ok(())
}
