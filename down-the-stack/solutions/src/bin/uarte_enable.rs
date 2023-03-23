#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use dk_pac::UARTE0;

// this imports `down-the-stack/apps/lib.rs` to retrieve our global logger + panicking-behavior
use solutions as _;
use defmt;
use defmt_rtt as _; // global logger

#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`

    // takes ownership of the nRF52840-DK peripherals
    let periph = dk_pac::Peripherals::take().unwrap();
    let uarte = periph.UARTE0;

    is_uarte_enabled(&uarte);

    // enable the UART0 peripheral the safe way
    uarte.enable.write(|w| w.enable().enabled());

    is_uarte_enabled(&uarte);

    // disable the UART0 peripheral by writing 0 directly into the register -- the unsafe way
    unsafe {
        uarte.enable.write(|w| w.bits(0x00u32));
    }

    is_uarte_enabled(&uarte);

    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}

fn is_uarte_enabled(uarte: &UARTE0) {
    if uarte.enable.read().enable().is_enabled() {
        defmt::println!("Uarte0 is enabled");
    } else {
        defmt::println!("Uarte0 is disabled");
    }
}