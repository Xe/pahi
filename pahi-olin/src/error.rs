use std::error;
use std::fmt;
use std::io;

// https://github.com/CommonWA/cwa-spec/blob/master/errors.md
pub const UNKNOWN: i32 = -1;
pub const INVALID_ARGUMENT: i32 = -2;
pub const PERMISSION_DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;
pub const EOF: i32 = -5;

/// An error abstraction, all of the following values are copied from the spec at:
/// https://github.com/CommonWA/cwa-spec/blob/master/errors.md
#[repr(i32)]
#[derive(Debug)]
pub enum Error {
    Unknown = UNKNOWN,
    InvalidArgument = INVALID_ARGUMENT,
    PermissionDenied = PERMISSION_DENIED,
    NotFound = NOT_FOUND,
    EOF = EOF,
}

impl Error {
    pub fn check(n: i32) -> Result<i32, Error> {
        match n {
            n if n >= 0 => Ok(n),
            INVALID_ARGUMENT => Err(Error::InvalidArgument),
            PERMISSION_DENIED => Err(Error::PermissionDenied),
            NOT_FOUND => Err(Error::NotFound),
            EOF => Err(Error::EOF),
            _ => Err(Error::Unknown),
        }
    }
}

/// pretty-print the error using Debug-derived code
/// XXX(Xe): is this a mistake?
impl self::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl self::error::Error for Error {}

pub fn check_io(error: i32) -> Result<i32, io::ErrorKind> {
    match error {
        n if n >= 0 => Ok(n),
        INVALID_ARGUMENT => Err(io::ErrorKind::InvalidInput),
        PERMISSION_DENIED => Err(io::ErrorKind::PermissionDenied),
        NOT_FOUND => Err(io::ErrorKind::NotFound),
        EOF => Err(io::ErrorKind::UnexpectedEof),
        _ => Err(io::ErrorKind::Other),
    }
}
