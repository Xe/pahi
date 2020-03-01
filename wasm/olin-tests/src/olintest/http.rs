extern crate http;
extern crate olin;
extern crate std;

use olin::log;
use std::vec::Vec;

pub extern "C" fn test() -> Result<(), i32> {
    log::info("running olintest::http tests");

    let mut resp_body = Vec::<u8>::new();
    let mut req_body = Vec::<u8>::new();
    let req = http::Request::builder()
        .uri("https://xena.greedo.xeserv.us/")
        .header("User-Agent", "Olin/dev")
        .header("Host", "xena.greedo.xeserv.us")
        .body(&mut req_body)
        .map_err(|e| {
            log::error(&format!("request error: {:?}", e));
            1
        })?;

    let resp =
        olin::http::client::transport("xena.greedo.xeserv.us".to_string(), req, &mut resp_body)
            .map_err(|e| {
                log::error(&format!("transport error: {:?}", e));
                1
            })?;

    log::info(&format!("status: {:?}", resp.status()));
    log::info(&format!(
        "response body: {}",
        std::str::from_utf8(&resp_body).unwrap()
    ));

    log::info("olintest::http tests passed");
    Ok(())
}
