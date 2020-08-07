extern crate olin;

use olin::Resource;
use std::io::Write;
use log::{error, info};

pub extern "C" fn test() -> Result<(), i32> {
    info!("testing for issue 39: https://github.com/Xe/olin/issues/39");

    const ZERO_LEN: usize = 16;
    let zeroes = [0u8; ZERO_LEN];
    let mut fout: Resource = Resource::open("null://").map_err(|e| {
        error!("can't open file: {:?}", e);
        1
    })?;
    let res = fout.write(&zeroes).map_err(|e| {
        error!("can't write: {:?}", e);
        1
    })?;

    if res != ZERO_LEN {
        error!("wanted res to be {} but got: {}", ZERO_LEN, res);
        return Err(1);
    }

    info!("issue 39 test passed");

    Ok(())
}
