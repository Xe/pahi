extern crate olin;

use log::{error, info};
use olin::startup;

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/ns/startup.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::startup tests");

    info!("checking argc/argv");
    let argc: i32 = startup::arg_len();
    info!("argc: {}", argc);

    for x in 0..argc {
        let mut arg_val = [0u8; 64];
        let arg = startup::arg_at_buf(x, &mut arg_val).ok_or_else(|| {
            error!("arg {} missing", x);
            1
        })?;
        info!("arg {}: {}", x, arg);
    }
    info!("passed");

    info!("ns::startup tests passed");
    Ok(())
}
