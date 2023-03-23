#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler] // called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle] // 
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

