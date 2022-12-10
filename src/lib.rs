#![no_std]
#![no_main]
#![feature(error_in_core)]

use core::fmt::Write;
use core::panic::PanicInfo;

pub mod bodaci_core;
use bodaci_core::{
    multiboot::{Tag, TagIterator},
    vga,
};

#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    println!("{}", i);
    loop {}
}

#[no_mangle]
unsafe extern "C" fn kmain(_multiboot_magic: u64, multiboot_addr: u64) -> ! {
    let tags = TagIterator::new(multiboot_addr + 8 as u64).unwrap();

    tags.for_each(|t| match t {
        Tag::BasicMemoryInfo(d) => {
            println!("{:#?}", d)
        }
        Tag::CommandLine(d) => {
            println!("{:#?}", d.s());
        }
        _ => (),
    });

    loop {}
}
