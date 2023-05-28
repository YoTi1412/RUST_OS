#![no_std] // don't link the rust std library
#![no_main] // disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(yoti_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use yoti_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
//    use x86_64::{structures::paging::Page, VirtAddr};
//    use yoti_os::memory;
    use yoti_os::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");
    yoti_os::init();

//    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
//    let mut mapper = unsafe { memory::init(phys_mem_offset) };
//    let mut frame_allocator = memory::EmptyFrameAllocator;

//    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
//    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

//    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
//    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // as before
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

