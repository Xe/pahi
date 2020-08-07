extern crate olin;

use olin::Resource;
use std::io::{Read, Write};
use log::{info, error};

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/schemes/null.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running scheme::null tests");

    let mut fout: Resource = Resource::open("null://").map_err(|e| {
        error!("couldn't open: {:?}", e);
        1
    })?;

    fout.write(b"entering rooms of fossil-light").map_err(|e| {
        error!("couldn't write: {:?}", e);
        1
    });

    let mut inp = [0u8; 16];
    fout.read(&mut inp).map_err(|e| {
        error!("couldn't read: {:?}", e);
        1
    });

    info!("flushing null file");
    fout.flush().map_err(|e| {
        error!("error: {:?}", e);
        1
    })?;

    info!("scheme::null tests passed");
    Ok(())
}
