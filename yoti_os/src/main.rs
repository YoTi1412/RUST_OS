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
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let ptr = 0x204d26 as *mut u32;

    unsafe { let x = *ptr; }
    println!("read worked");
    
    unsafe { *ptr = 42; }
    println!("write worked");


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

