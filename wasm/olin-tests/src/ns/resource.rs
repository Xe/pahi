extern crate olin;

use olin::Resource;
use std::io::{Read, Write};
use log::{error, info};

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/ns/runtime.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::resource tests");

    info!("trying to open a log:// file");
    {
        let mut fout: Resource =
            Resource::open("log://?prefix=test-log-please-ignore").map_err(|e| {
                error!("couldn't open: {:?}", e);
                1
            })?;

        let res = fout.write(b"hi from inside log file");
        if let Err(err) = res {
            error!("can't write message to log file: {}", err);
        }
    }
    info!("successfully closed the file");

    info!("opening a zero:// file");
    {
        let mut fout: Resource = Resource::open("zero://").map_err(|e| {
            error!("error: {:?}", e);
            1
        })?;

        info!("reading zeroes");
        let mut zeroes = [0u8; 16];
        let res = fout.read(&mut zeroes);
        if let Err(err) = res {
            error!("can't read zeroes from zero file: {}", err);
            return Err(1);
        }

        info!("verifying all zeroes are valid");
        for (x, val) in zeroes.iter().enumerate() {
            let val: u8 = *val;
            if val != 0 {
                error!("expected zeroes[{}] to be 0, got: {}", x, val);
                return Err(1);
            }
        }

        info!("flushing zero file");
        fout.flush().map_err(|e| {
            error!("error: {:?}", e);
            1
        })?;
    }
    info!("closed file");

    info!("ns::resource tests passed");
    Ok(())
}
