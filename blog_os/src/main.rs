#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
use core::panic::PanicInfo;

#[panic_handler] // called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello World, Im YoTi";

#[no_mangle] // don't mangle the name of this fn
pub extern "C" fn _start() -> ! {
    // this fn is the entry point, since the linker looks
    // for a fn called "_start" by defaulf
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

