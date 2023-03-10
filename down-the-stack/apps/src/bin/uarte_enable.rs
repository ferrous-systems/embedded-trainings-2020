#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use dk_pac;

// this imports `down-the-stack/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;
use defmt;
use defmt_rtt as _; // global logger


#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`
    unsafe {
    let periph = dk_pac::Peripherals::steal();
    let uarte = periph.UARTE0;
    
    uarte.enable.write(|w| w.enable().enabled());
   
    if uarte.enable.read().bits() != 0 {
        defmt::println!("Uarte0 is enabled");

    } else {
        defmt::println!("Uarte0 is disabled");
    }

    }
    
    
    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}