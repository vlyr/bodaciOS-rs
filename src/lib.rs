#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

pub mod bodaci_core;
use bodaci_core::vga::{self, Color};

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn kmain(multiboot_magic: u64, multiboot_addr: u64) -> ! {
    vga::write("Hello, welcome to BodaciOS\n\n");

    vga::write_colored(
        "Here's a test for colored text.\n",
        (Color::Yellow, Color::Black),
    );

    println!("Hey {:#?}", multiboot_magic);

    loop {}
}
