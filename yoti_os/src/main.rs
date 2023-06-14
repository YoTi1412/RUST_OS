#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(yoti_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use yoti_os::println;

entry_point!(kernel_main);

extern crate alloc;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
    use x86_64::VirtAddr;
    use yoti_os::allocator;
    use yoti_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    yoti_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

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
