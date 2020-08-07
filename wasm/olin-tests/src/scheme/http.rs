extern crate httparse;
extern crate olin;

use log::{error, info};
use olin::{http, Resource};
use std::io::{Read, Write};

pub extern "C" fn test() -> Result<(), i32> {
    info!("running scheme::http tests");

    let reqd = "GET /404 HTTP/1.1\r\nHost: xena.greedo.xeserv.us\r\nUser-Agent: Bit-banging it in rust\r\n\r\n";
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    info!("validating HTTP request");
    req.parse(reqd.as_bytes()).map_err(|e| {
        error!("can't parse request: {:?}", e);
        1
    });

    info!("opening https://xena.greedo.xeserv.us");
    let mut fout: Resource = Resource::open("https://xena.greedo.xeserv.us").map_err(|e| {
        error!("couldn't open: {:?}", e);
        1
    })?;

    info!("writing HTTP request");
    fout.write(reqd.as_bytes()).map_err(|e| {
        error!("can't write request: {:?}", e);
        1
    });

    info!("fetching response");
    fout.flush().map_err(|e| {
        error!("can't send request to remote server: {:?}", e);
        1
    });

    info!("reading response");
    let mut resp_data = [0u8; 2048];
    fout.read(&mut resp_data).map_err(|e| {
        error!("can't read response: {:?}", e);
        1
    });

    info!("parsing response");
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut resp = httparse::Response::new(&mut headers);
    resp.parse(&resp_data).map_err(|e| {
        error!("can't parse response: {:?}", e);
        1
    });

    info!(
        "version: {:?}, code: {:?}, reason: {:?}",
        resp.version, resp.code, resp.reason
    );

    info!("scheme::http tests passed");
    Ok(())
}
