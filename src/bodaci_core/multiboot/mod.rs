pub mod error;
use error::{Error, Result};

pub const CODE_END: u32 = 0;
pub const CODE_BOOT_COMMAND_LINE: u32 = 1;
pub const CODE_BASIC_MEMORY_INFO: u32 = 4;
pub const CODE_BIOS_BOOT_DEVICE: u32 = 5;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Header {
    pub magic: u32,
    pub architecture: u32,
    pub header_length: u32,
    pub checksum: u32,
}

pub mod tags {
    use super::error::{Error, Result};
    use crate::{print, println};
    use core::mem;
    use core::slice;
    use core::str;

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct GenericTag {
        pub code: u32,
        pub size: u32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct APMTableTag {
        pub code: u32,
        pub size: u32,
        pub version: u16,
        pub cseg: u16,
        pub offset: u32,
        pub cseg_16: u16,
        pub dseg: u16,
        pub flags: u16,
        pub cseg_len: u16,
        pub cseg_16_len: u16,
        pub dseg_len: u16,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BasicMemoryInfoTag {
        pub code: u32,
        pub size: u32,
        /// Amount of lower memory (in kilobytes)
        pub mem_lower: u32,
        /// Amount of upper memory (in kilobytes)
        pub mem_upper: u32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BIOSBootDeviceTag {
        pub code: u32,
        pub size: u32,
        pub biosdev: u32,
        pub partition: u32,
        pub sub_partition: u32,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct CommandLineTag {
        pub code: u32,
        pub size: u32,
        string: u8,
    }

    impl CommandLineTag {
        pub fn s(&self) -> Option<&str> {
            let str_len = self.size as usize - mem::size_of::<CommandLineTag>() + 1;
            let data = unsafe { slice::from_raw_parts((&self.string) as *const u8, str_len) };

            if data.len() <= 1 {
                return None;
            }

            str::from_utf8(data).ok()
        }
    }
}

pub use tags::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Tag {
    APMTable(APMTableTag),
    BIOSBootDevice(BIOSBootDeviceTag),
    BasicMemoryInfo(BasicMemoryInfoTag),
    CommandLine(CommandLineTag),
    Unknown,
}

impl Tag {
    pub unsafe fn from_ptr(ptr: *const u64) -> Self {
        let data: GenericTag = (ptr as *const GenericTag).read();

        match data.code {
            1 => Tag::CommandLine(ptr.cast::<CommandLineTag>().read()),
            4 => Tag::BasicMemoryInfo((ptr.cast::<BasicMemoryInfoTag>()).read()),
            5 => Tag::BIOSBootDevice((ptr.cast::<BIOSBootDeviceTag>()).read()),
            10 => Tag::APMTable((ptr.cast::<APMTableTag>()).read()),
            _ => Tag::Unknown,
        }
    }
}

/// A struct that parses multiboot tags from an address and returns an iterator over them.
pub struct TagIterator {
    addr: u64,
}

impl TagIterator {
    /// Constructs a TagIterator from a starting address
    /// NOTE: The address must be 8-byte aligned.
    pub fn new(addr: u64) -> Result<Self> {
        if addr % 8 != 0 {
            Err(Error::InvalidAddress)
        } else {
            Ok(Self { addr })
        }
    }
}

impl Iterator for TagIterator {
    type Item = Tag;

    fn next(&mut self) -> Option<Tag> {
        let tag_data = unsafe { (self.addr as *const GenericTag).read() };

        let res = match tag_data {
            GenericTag { code: 0, size: 8 } => None,
            _ => Some(unsafe { Tag::from_ptr(self.addr as *const _) }),
        };

        self.addr += (tag_data.size as u64 + 7) & !7;

        res
    }
}

mod seal {
    pub trait Sealed {}
}
