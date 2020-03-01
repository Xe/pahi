use crate::{error::Error, resource::Resource};
use log::error;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use url::Url;

pub struct Http {
    stream: TcpStream,
}

impl Resource for Http {
    fn new(u: Url) -> Result<Http, Error> {
        if let None = u.host() {
            return Err(Error::InvalidArgument);
        }

        let host = u.host().unwrap();
        let port: u16 = match u.port() {
            Some(port_num) => port_num,
            None => 80,
        };

        TcpStream::connect((host.to_string().as_str(), port))
            .or_else(|why| {
                error!(
                    "connection error to {}:{}: {:?}",
                    u.host().unwrap(),
                    port,
                    why
                );
                Err(Error::Unknown)
            })
            .and_then(|conn| Ok(Http { stream: conn }))
    }

    fn close(&mut self) {
        if let Err(why) = self.stream.shutdown(Shutdown::Both) {
            error!("error closing TCP stream: {:?}", why);
        }

        drop(&self.stream);
    }
}

impl Read for Http {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for Http {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.shutdown(Shutdown::Write)
    }
}
