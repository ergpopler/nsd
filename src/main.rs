#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);

}


#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(11, 1);
    serial_println!("[ok]");
}


mod vga_buffer;
mod serial;

static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;

// This gets called on panic.

#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
	println!("An error has occured: {}", i);
    loop {}
}



// this prevents the name from being mangled.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, println{}", "!");

	#[cfg(test)]
    test_main();

    loop {
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;
	
	unsafe {
	    let mut port = Port::new(0xf4);
	    port.write(exit_code as u32);
	}
}
