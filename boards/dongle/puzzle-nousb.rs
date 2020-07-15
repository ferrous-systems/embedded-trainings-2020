#![deny(unused_must_use)]
#![no_main]
#![no_std]

use hal::{
    led,
    radio::{self, Channel, Packet},
};
use heapless::{consts, LinearMap};
use panic_abort as _;

static FROM: &[u8] = &[
    // <redacted>
];

static TO: &[u8] = &[
    // <redacted>
];

// store the secret rather than the plaintext -- otherwise `strings $elf` will reveal the answer
static SECRET: &[u8] = b"<redacted>";

#[no_mangle]
fn main() -> ! {
    let (mut rtx, mut rrx) = radio::claim(Channel::_26);
    let led = led::Green;

    let mut dict = LinearMap::<_, _, consts::U128>::new();
    for (&from, &to) in FROM.iter().zip(TO.iter()) {
        dict.insert(from, to).ok();
    }

    let task = async {
        let mut packet = Packet::new().await;
        let mut on = true;

        loop {
            let crcres = rrx.read(&mut packet).await;
            // toggle LED on each new packet
            if on {
                led.on();
            } else {
                led.off();
            }
            on = !on;

            if crcres.is_ok() {
                if packet.is_empty() {
                    packet.copy_from_slice(SECRET);
                } else if packet.len() == 1 {
                    let p = packet[0];
                    let c = dict.get(&p).unwrap_or(&p);
                    packet.copy_from_slice(&[*c]);
                } else {
                    // encrypt
                    for slot in packet.iter_mut() {
                        if let Some(c) = dict.get(slot) {
                            *slot = *c;
                        }
                    }

                    let matches = &packet[..] == SECRET;
                    packet.copy_from_slice(if matches {
                        led::Blue.on();
                        b"correct"
                    } else {
                        led::Blue.off();
                        b"incorrect"
                    });
                }

                rtx.write(&packet).await.ok();
            }
        }
    };

    executor::run!(task)
}
