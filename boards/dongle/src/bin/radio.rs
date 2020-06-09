//! Test program, mainly for debugging purposes

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{OutputPin as _, StatefulOutputPin as _};
use hal::{
    clocks::Clocks,
    gpio::{p0, Level},
    ieee802154::{Packet, Radio},
    target as pac,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let port = p0::Parts::new(periph.P0);

    // NOTE LEDs turn on when the output level is low (0V)
    let mut green = port.p0_06.into_push_pull_output(Level::High); // LD1

    let mut blue = port.p0_12.into_push_pull_output(Level::High); // LD2 (RGB LED)
    let mut red = port.p0_08.into_push_pull_output(Level::High); // LD2 (RGB LED)

    let mut radio = Radio::init(periph.RADIO, &clocks);

    // turn on green LED to indicate the radio has been initialized
    green.set_low().ok();

    // set TX power to its maximum value
    radio.set_txpower(8);

    let mut packet = Packet::new();
    loop {
        let res = radio.recv(&mut packet);

        if res.is_ok() {
            // CRC check passed
            // clear visual error state (turn off the red LED)
            red.set_high().ok();

            // toggle blue LED on each successfully received packet
            if blue.is_set_low() == Ok(true) {
                blue.set_high().ok();
            } else {
                blue.set_low().ok();
            }

            // return packet with the contents unchanged
            radio.send(&packet);
        } else {
            // CRC check failed
            // indicate error using the red LED
            red.set_low().ok();
        }
    }
}
