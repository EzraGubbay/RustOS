#![no_std] // Disable use of Standard Library
#![no_main] // Disable search for main function

mod vga_buffer;

// Struct for information on a panicked program.
use core::panic::PanicInfo;

// Custom entry point _start, using C Calling Convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");
    
    loop {}
}

// Custom panic handler, as we cannot use the one in Standard Library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
