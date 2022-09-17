#![no_std]
#![no_main]

use rust_os::vga_buffer;
use rust_os::interrupts;
use rust_os::println;

use core::panic::PanicInfo;

static OS_NAME: &'static str = "SmallOS";

pub fn init() {
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    vga_buffer::change_color(
        vga_buffer::Color::Yellow, vga_buffer::Color::DarkGray);
    vga_buffer::clear_screen();    
    
    println!("Starting {}", OS_NAME);
    println!("Another line.");

    x86_64::instructions::interrupts::int3();

    println!("Made it here.");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic detected: {}", &info);
    
    loop{}
}
