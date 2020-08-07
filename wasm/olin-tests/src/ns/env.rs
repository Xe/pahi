extern crate olin;

use log::{error, info};
use olin::env;
use std::str;

/// This tests for https://github.com/CommonWA/cwa-spec/blob/master/ns/env.md
pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::env tests");

    info!("env[\"MAGIC_CONCH\"] = \"yes\"");
    let envvar_name = "MAGIC_CONCH";
    let mut envvar_val = [0u8; 64];
    let envvar_val = env::get_buf(envvar_name.as_bytes(), &mut envvar_val)
        .map(|s| str::from_utf8(s).expect("envvar wasn't UTF-8"))
        .map_err(|e| {
            error!("couldn't get: {:?}", e);
            1
        })?;

    if envvar_val != "yes" {
        error!("wanted yes, got: {}", envvar_val);
        return Err(1);
    }
    info!("passed");

    info!("look for variable that does not exist");
    match env::get("DOES_NOT_EXIST") {
        Err(env::Error::NotFound) => info!("this does not exist! :D"),
        Ok(_) => {
            error!("DOES_NOT_EXIST exists");
            return Err(1);
        }
        _ => {
            error!("other error");
            return Err(2);
        }
    }

    info!("ns::env tests passed");
    Ok(())
}
