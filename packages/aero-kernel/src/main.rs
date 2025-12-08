// Aero OS Kernel - Bootstrapped Entry Point
//
// This is a placeholder. The actual kernel will be written in Fruti
// once the compiler is complete.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel initialization will go here
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
