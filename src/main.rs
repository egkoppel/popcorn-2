#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
	loop {

	}
}
