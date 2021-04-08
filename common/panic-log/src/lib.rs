#![no_std]

use core::panic::PanicInfo;

use cortex_m::asm;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::error!("{}", defmt::Debug2Format(&info));

    // abort instruction: triggers a HardFault exception which causes probe-run to exit
    asm::udf()
}
