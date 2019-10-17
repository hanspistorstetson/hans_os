#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hans_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use hans_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hans_os::memory;
    use hans_os::allocator;
    use x86_64::VirtAddr;

    println!("Hello world!");
    hans_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");


    let heap_value = Box::new(42);
    println!("heap value at {:p}", heap_value);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());



    #[cfg(test)]
    test_main();

    println!("Program did not crash!");

    hans_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hans_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hans_os::test_panic_handler(info)
}
