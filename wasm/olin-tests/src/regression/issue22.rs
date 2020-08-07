extern crate olin;

use olin::env;
use log::{error, info};

pub extern "C" fn test() -> Result<(), i32> {
    info!("testing for issue 22: https://github.com/Xe/olin/issues/22");

    info!("look for variable that does not exist");
    match env::get("DOES_NOT_EXIST") {
        Err(env::Error::NotFound) => info!("this does not exist! :D"),
        Ok(_) => {
            error!("DOES_NOT_EXIST exists");

            return Err(1);
        }
        _ => {
            error!("got other error");

            return Err(2);
        }
    }

    info!("issue 22 test passed");

    Ok(())
}
