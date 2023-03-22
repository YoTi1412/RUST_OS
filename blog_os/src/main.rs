#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
use core::panic::PanicInfo;

#[panic_handler] // called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // don't mangle the name of this fn
pub extern "C" fn _start() -> ! {
    // this fn is the entry point, since the linker looks
    // for a fn called "_start" by defaulf
    loop{}
}

