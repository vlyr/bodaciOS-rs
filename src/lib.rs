#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

pub mod vga;
use vga::Color;

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn kmain(multiboot_magic: u64, multiboot_addr: u64) -> ! {
    let mut vga_pos = vga::Position::default();

    vga::write("Hello, welcome to BodaciOS", &mut vga_pos);
    vga::write_colored("hey", (Color::Yellow, Color::Black), &mut vga_pos);
    vga::write_fmt(format_args!("| {}", multiboot_addr), &mut vga_pos).unwrap();

    loop {}
}
