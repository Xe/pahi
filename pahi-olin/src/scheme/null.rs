use crate::{error::Error, resource::Resource};
use std::io::{self, Read, Write};
use url::Url;

pub struct Null {}

impl Resource for Null {
    fn new(_: Url) -> Result<Null, Error> {
        Ok(Null {})
    }

    fn close(&mut self) {}
}

impl Read for Null {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
}

impl Write for Null {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
