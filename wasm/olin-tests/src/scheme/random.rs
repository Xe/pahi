extern crate olin;

use log::{error, info};
use olin::Resource;
use std::io::{Read, Write};

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/schemes/random.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running scheme::random tests");

    let mut fin: Resource = Resource::open("random://").map_err(|e| {
        error!("couldn't open: {:?}", e);
        1
    })?;

    let mut inp = [0u8; 16];
    fin.read(&mut inp).map_err(|e| {
        error!("couldn't read: {:?}", e);
        1
    })?;

    info!("flushing random file");
    fin.flush().map_err(|e| {
        error!("error: {:?}", e);
        1
    })?;

    info!("scheme::random tests passed");
    Ok(())
}
