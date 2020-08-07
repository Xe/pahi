extern crate http;
extern crate olin;
extern crate std;

use std::vec::Vec;
use log::{error, info};

pub extern "C" fn test() -> Result<(), i32> {
    info!("running olintest::http tests");

    let mut resp_body = Vec::<u8>::new();
    let mut req_body = Vec::<u8>::new();
    let req = http::Request::builder()
        .uri("https://xena.greedo.xeserv.us/")
        .header("User-Agent", "Olin/dev")
        .header("Host", "xena.greedo.xeserv.us")
        .body(&mut req_body)
        .map_err(|e| {
            error!("request error: {:?}", e);
            1
        })?;

    let resp =
        olin::http::client::transport("xena.greedo.xeserv.us".to_string(), req, &mut resp_body)
            .map_err(|e| {
                error!("transport error: {:?}", e);
                1
            })?;

    info!("status: {:?}", resp.status());
    info!(
        "response body: {}",
        std::str::from_utf8(&resp_body).unwrap()
    );

    info!("olintest::http tests passed");
    Ok(())
}
