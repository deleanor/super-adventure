#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static OS_NAME: &'static str = "SmallOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::change_color(
        vga_buffer::Color::Yellow, vga_buffer::Color::Blue);
    vga_buffer::clear_screen();    
    
    println!("Starting {}", OS_NAME);
    println!("Another line.");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic detected: {}", &info);
    
    loop{}
}