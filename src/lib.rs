#![no_std]
#![no_main]
#![feature(error_in_core)]
#![feature(exclusive_range_pattern)]

use core::fmt::Write;
use core::panic::PanicInfo;

pub mod bodaci_core;
use bodaci_core::{
    multiboot::{Tag, TagIterator},
    vga::{self, Color},
};

#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    println!("{}", i);
    loop {}
}

const COLOR_TEST_MESSAGE: &str = "Welcome - BodaciOS";

#[no_mangle]
unsafe extern "C" fn kmain(_multiboot_magic: u64, multiboot_addr: u64) -> ! {
    let tags = TagIterator::new(multiboot_addr + 8 as u64).unwrap();

    ('\x10'..'\x20').enumerate().for_each(|(idx, c)| {
        print!("{}{}", c, COLOR_TEST_MESSAGE.chars().nth(idx).unwrap());
    });

    println!("\n");

    println!(
        "Test {}yellow text {}white text",
        Color::Yellow,
        Color::White
    );

    tags.for_each(|t| match t {
        Tag::BasicMemoryInfo(d) => {
            println!("{:#?}", d)
        }
        Tag::CommandLine(d) => {
            println!("{:#?}", d.string());
        }
        _ => (),
    });

    loop {}
}
