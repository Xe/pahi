extern crate olin;

use olin::runtime;
use log::{error, info};

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/ns/runtime.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::runtime tests");

    info!("expecting spec major=0 and min=0");
    let minor: i32 = runtime::spec_major();
    let major: i32 = runtime::spec_major();

    info!("minor: {}, major: {}", minor, major);

    if major != 0 && minor != 0 {
        error!("version is wrong");
        return Err(1);
    }
    info!("passed");

    info!("sleeping");
    runtime::msleep(1);
    info!("passed");

    info!("getting runtime name, should be olin or pa'i");
    let mut rt_name = [0u8; 32];
    let runtime_name = runtime::name_buf(rt_name.as_mut()).ok_or_else(|| {
        error!("Runtime name larger than 32 byte limit");
        1
    })?;

    info!("{}", runtime_name);

    match runtime_name.to_string().as_str() {
        "olin" | "pa'i" => {
            info!("passed");
            info!("ns::runtime tests passed");
            Ok(())
        }
        _ => {
            error!("Got runtime name, not olin or pa'i");
            return Err(1);
        }
    }
}
