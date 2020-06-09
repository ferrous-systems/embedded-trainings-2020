#![no_std]

use core::panic::PanicInfo;

use cortex_m::asm;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    loop {
        asm::bkpt()
    }
}
