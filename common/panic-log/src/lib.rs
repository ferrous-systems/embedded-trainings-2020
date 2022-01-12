#![no_std]

use core::panic::PanicInfo;

use cortex_m::asm;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { (0x4002_7504 as *mut u32).write_volatile(0) }

    log::error!("{}", info);

    // abort instruction: triggers a HardFault exception which causes probe-run to exit
    asm::udf()
}
