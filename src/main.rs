#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nsd::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nsd::println;

#[inline(always)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	let mut x: i32 = 0;
    loop {
    	println!("{}", x);
    	x += 1;
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nsd::test_panic_handler(info)
}
