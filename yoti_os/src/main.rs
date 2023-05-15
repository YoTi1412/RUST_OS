#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(yoti_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use yoti_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    yoti_os::init();


    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    yoti_os::hlt_loop();
}

// this function is called a panic ------
#[cfg(not(test))]
#[panic_handler] // called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    yoti_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yoti_os::test_panic_handler(info)
}

