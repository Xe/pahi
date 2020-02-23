use std::io::{self, Read, Write};

pub trait Resource: Read + Write {
    fn new(location: String) -> Self;

    fn close(&mut self);
}

pub struct Stdin {
    inp: ::std::io::Stdin,
}

impl Resource for Stdin {
    fn new(_: String) -> Stdin {
        Stdin { inp: io::stdin() }
    }

    fn close(&mut self) {}
}

impl Write for Stdin {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut handle = self.inp.lock();
        handle.read(buf)
    }
}

pub struct Stdout {
    out: io::Stdout,
}

impl Resource for Stdout {
    fn new(_: String) -> Stdout {
        Stdout { out: io::stdout() }
    }

    fn close(&mut self) {}
}

impl Read for Stdout {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut handle = self.out.lock();
        handle.write_all(buf)?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut handle = self.out.lock();
        handle.flush()?;

        Ok(())
    }
}

pub struct Stderr {
    err: io::Stderr,
}

impl Resource for Stderr {
    fn new(_: String) -> Stderr {
        Stderr { err: io::stderr() }
    }

    fn close(&mut self) {}
}

impl Read for Stderr {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut handle = self.err.lock();
        handle.write_all(buf)?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut handle = self.err.lock();
        handle.flush()?;

        Ok(())
    }
}
