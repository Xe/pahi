use crate::resource::Resource;
use log::{error, info};
use std::io::{self, Error, ErrorKind, Read, Write};
use url::Url;

pub struct Log {}

impl Resource for Log {
    fn new(_: Url) -> Result<Log, crate::error::Error> {
        Ok(Log {})
    }
    fn close(&mut self) {}
}

impl Read for Log {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
}

impl Write for Log {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(msg) => {
                info!("log://: {}", msg);
                Ok(buf.len())
            }
            Err(why) => {
                error!("log write error: {}", why);
                Err(Error::new(ErrorKind::Other, why))
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
