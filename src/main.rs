#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points


static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;

// This gets called on panic.

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
	loop{
		
	}
}


// this prevents the name from being mangled.

#[no_mangle]
pub extern "C" fn _start() -> ! {

	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
		
	}
    loop {
    	
    }
}