#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;

#[panic_handler]
fn panic_info(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world! ");
    panic!("Some panic message");
    loop {}
}