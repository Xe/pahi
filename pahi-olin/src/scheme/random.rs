use crate::resource::Resource;
use rand::prelude::*;
use std::io::{self, Read, Write};
use url::Url;

pub struct Random {}

impl Resource for Random {
    fn new(_: Url) -> Random {
        Random {}
    }

    fn close(&mut self) {}
}

impl Read for Random {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut rng = thread_rng();
        rng.fill_bytes(buf);
        Ok(buf.len())
    }
}

impl Write for Random {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
