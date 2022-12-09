use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidAddress,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::InvalidAddress => "Invalid Address",
        };

        write!(f, "{}", msg)
    }
}

impl core::error::Error for Error {}
