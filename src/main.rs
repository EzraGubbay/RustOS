#![no_std] // Disable use of Standard Library
#![no_main] // Disable search for main function
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod vga_buffer;

// Struct for information on a panicked program.
use core::panic::PanicInfo;

// Custom entry point _start, using C Calling Convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Custom panic handler, as we cannot use the one in Standard Library
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Test function
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    print!("[ok]");
}