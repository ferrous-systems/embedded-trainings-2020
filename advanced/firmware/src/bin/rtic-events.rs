#![no_main]
#![no_std]

use cortex_m::asm;
use panic_log as _; // panic handler

#[rtfm::app(device = dk)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        let board = dk::init().unwrap();

        // `POWER` is a peripheral, or register block
        let power = board.power;

        // INTENSET is one of POWER's registers
        // the `write` method writes a (32-bit) value into the register
        power.intenset.write(|w| {
            // `w` is a "constructor" with methods to clear/set the bitfields of INTENSET
            // `w` starts with all bitfields set to their reset value
            // USBDETECTED is one of INTENSET's 1-bit fields
            w.usbdetected().set_bit()
        });

        log::info!("USBDETECTED interrupt enabled");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        log::info!("idle: going to sleep");

        // sleep in the background
        loop {
            asm::wfi();
        }
    }

    #[task(binds = POWER_CLOCK)]
    fn on_power_event(_cx: on_power_event::Context) {
        log::info!("POWER event occurred");
        dk::exit()
    }
};
