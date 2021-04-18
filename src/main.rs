#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kitty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kitty_os::println;


#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic_info(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kitty_os::test_panic_handler(info);
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world! ");

    #[cfg(test)] test_main();

    loop {}
}

