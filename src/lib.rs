#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    let buffer = (0xb8000) as *mut u16;

    *buffer.offset(0) = 0x0F21;
    loop {}
}
