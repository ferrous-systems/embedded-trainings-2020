#![no_main]
#![no_std]

use cortex_m::asm;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        let board = dk::init().unwrap();

        // `POWER` is a peripheral, or register block
        let power = board.power;

        // INTENSET is one of POWER's registers
        // the `write()` method writes a (32-bit) value into the register
        power.intenset.write(|w| {
            // `w` is a "constructor" with methods to clear/set the bitfields of INTENSET
            // `w` starts with all bitfields set to their reset value
            // USBDETECTED is one of INTENSET's 1-bit fields
            w.usbdetected().set_bit()
        });

        defmt::info!("USBDETECTED interrupt enabled");

        // read the whole 32-bit usb supply register
        // the `read()` method returns a reader which can then be used to access the register content
        // in full or only specific bitfields (see below)
        // (the layout of the USBREGSTATUS register can be found in section 5.3.7.13 of the PS)
        let regstatus: u32 = power.usbregstatus.read().bits();
        //                                             ^^^^ complete register content
        defmt::info!("USBREGSTATUS: {:b}", regstatus);

        // read the 1-bit VBUSDETECT field that is part of the USBREGSTATUS register content
        // to show that its contents reflect our usb connection status
        // (the USBDETECTED event that will trigger `on_power_event()` is derived from this information)
        let vbusdetect: bool = power.usbregstatus.read().vbusdetect().bits();
        //                                               ^^^^^^^^^^ bitfield name
        defmt::info!("USBREGSTATUS.VBUSDETECT: {}", vbusdetect);
    }

    #[idle]
    fn main(_cx: main::Context) -> ! {
        defmt::info!("idle: going to sleep");

        // sleep in the background
        loop {
            asm::wfi();
        }
    }

    #[task(binds = POWER_CLOCK)]
    fn on_power_event(_cx: on_power_event::Context) {
        defmt::info!("POWER event occurred");
        dk::exit()
    }
};
