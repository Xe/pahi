use crate::{error::Error, resource::Resource};
use std::io::{self, Read, Write};
use url::Url;

pub struct Zero {}

impl Resource for Zero {
    fn new(_: Url) -> Result<Zero, Error> {
        Ok(Zero {})
    }

    fn close(&mut self) {}
}

impl Read for Zero {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0..buf.len() {
            buf[i] = 0
        }

        Ok(buf.len())
    }
}

impl Write for Zero {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
