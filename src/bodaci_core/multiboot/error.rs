use core::fmt;

/// A wrapper around core::result::Result with multiboot::Error.
pub type Result<T> = core::result::Result<T, Error>;

/// The Error type for Multiboot operations.
#[derive(Debug, Clone)]
pub enum Error {
    /// The address was not 8-byte aligned.
    InvalidAddress,
    /// Either the memory given for the string was invalid, or the bootloader is not
    /// multiboot2-compliant
    InvalidStringData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::InvalidAddress => "The given address is not 8-byte aligned.",
            Error::InvalidStringData => "Either the memory given for the string was invalid, or the bootloader is not multiboot2-compliant.",
        };

        write!(f, "{}", msg)
    }
}

impl core::error::Error for Error {}
