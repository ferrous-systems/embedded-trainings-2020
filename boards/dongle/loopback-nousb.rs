#![deny(unused_must_use)]
#![no_main]
#![no_std]

use hal::{radio::{self, Channel}, led};
use panic_abort as _;

#[no_mangle]
fn main() -> ! {
    let (mut rtx, mut rrx) = radio::claim(Channel::_21); // <- change this
    let led = led::Green;

    let task = async {
        let mut packet = radio::Packet::new().await;
        let mut on = true;

        loop {
            let crcres = rrx.read(&mut packet).await;
            // togle LED on each new packet
            if on {
                led.on();
            } else {
                led.off();
            }
            on = !on;

            if crcres.is_ok() {
                packet.reverse();
                rtx.write(&packet).await.ok();
            }
        }
    };

    executor::run!(task)
}
