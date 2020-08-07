extern crate olin;

use log::{error, info};
use olin::Resource;
use std::io::Write;

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/schemes/log.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running scheme::log tests");

    info!("log://");
    {
        let mut fout: Resource = Resource::open("log://").map_err(|e| {
            error!("couldn't open: {:?}", e);
            1
        })?;

        fout.write(b"I am listening for a sound beyond sound")
            .map_err(|e| {
                error!("couldn't write log: {:?}", e);
                1
            });

        fout.flush().map_err(|e| {
            error!("couldn't flush: {:?}", e);
            1
        });
    }
    info!("passed");

    info!("log://?prefix=inner");
    {
        let mut fout: Resource = Resource::open("log://?prefix=inner").map_err(|e| {
            error!("couldn't open: {:?}", e);
            1
        })?;

        fout.write(b"that stalks the nightland of my dreams")
            .map_err(|e| {
                error!("couldn't write log: {:?}", e);
                1
            });

        fout.flush().map_err(|e| {
            error!("couldn't flush: {:?}", e);
            1
        });
    }
    info!("passed");

    info!("scheme::log tests passed");
    Ok(())
}
