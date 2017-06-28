use std::result;
use std::io;
use std::fmt;
use std::error;

use sincere;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Sincere(sincere::error::Error),
    CodedError(ErrorCode),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<sincere::error::Error> for Error {
    fn from(err: sincere::error::Error) -> Error {
        Error::Sincere(err)
    }
}

impl From<ErrorCode> for Error {
    fn from(err: ErrorCode) -> Error {
        Error::CodedError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::Sincere(ref inner) => inner.fmt(fmt),
            Error::CodedError(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::Sincere(ref err) => err.description(),
            Error::CodedError(ref err) => err.to_str(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::Sincere(ref err) => Some(err),
            Error::CodedError(_) => None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub struct ErrorCode(pub u16);

impl ErrorCode {
    pub fn to_str(&self) -> &str {
        match self.0 {
            _ => "未知错误"
        }
    }

    pub fn to_code(&self) -> u16 {
        self.0
    }
}

impl From<i16> for ErrorCode {
    fn from(in_code: i16) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u16> for ErrorCode {
    fn from(in_code: u16) -> ErrorCode {
        ErrorCode(in_code)
    }
}

impl From<i32> for ErrorCode {
    fn from(in_code: i32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }
}

impl From<u32> for ErrorCode {
    fn from(in_code: u32) -> ErrorCode {
        ErrorCode(in_code as u16)
    }

}

impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.to_str())
    }
}
