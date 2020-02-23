use crate::resource::Resource;
use std::io::{self, Read, Write};
use url::Url;

pub struct Null {}

impl Resource for Null {
    fn new(_: Url) -> Null {
        Null {}
    }

    fn close(&mut self) {}
}

impl Read for Null {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
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
