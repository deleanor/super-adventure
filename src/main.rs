#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static OS_NAME: &'static str = "SmallOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting {}", OS_NAME);
    println!("Here's another line.");
    println!("Foo");

    panic!("at the disco");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic detected: {}", &info);
    
    loop{}
}