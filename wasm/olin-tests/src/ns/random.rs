extern crate olin;

use olin::{random};
use log::info;

pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::random tests");
    info!("i31: {}, i63: {}", random::i31(), random::i63());
    info!("ns::random tests passed");
    Ok(())
}
