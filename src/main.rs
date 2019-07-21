#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

pub mod vga;
#[macro_use]
mod fmt;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    _halt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    _halt();
}

fn _halt() -> ! {
    println!("Hello, World!");
    println!("Testing testing...");

    for i in 0..50 {
        sleep();
        println!("i = {}", i);
    }

    loop {
        unsafe {
            asm!("cli; hlt");
        }
    }
}

fn sleep() {
    for i in 0..200000 {
    }
}