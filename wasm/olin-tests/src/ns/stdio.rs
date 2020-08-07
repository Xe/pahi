extern crate olin;

use olin::stdio;
use std::io::{Read, Write};
use log::{error, info};

pub extern "C" fn test() -> Result<(), i32> {
    info!("running ns::stdio tests");

    info!("stdout");
    {
        let mut fout = stdio::out();
        fout.write(b"Hi there\n").map_err(|e| {
            error!("can't write to stdout: {:?}", e);
            1
        });
    }

    info!("stderr");
    {
        let mut fout = stdio::err();
        fout.write(b"Hi there\n").map_err(|e| {
            error!("can't write to stderr: {:?}", e);
            1
        });
    }

    info!("stdin");
    {
        let mut fin = stdio::inp();
        let mut resp: [u8; 16] = [0u8; 16];
        fin.read(&mut resp).map_err(|e| {
            error!("can't read from stdin: {:?}", e);
            1
        });
    }

    info!("ns::stdio tests passed");
    Ok(())
}
