// f6c27c0a5af464795d1a64c45fa1b3039f44a62a
#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::fmt::Write as _;

use hal::{radio::{self, Packet, Channel}, usbd, led};
use heapless::{consts, LinearMap, String};
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
    // so we can visually differentiate this one from `loopback.hex`
    led::Green.on();

    let mut stx = usbd::serial();
    let (mut rtx, mut rrx) = radio::claim(Channel::_25);
    let mut output = String::<consts::U128>::new();

    let mut dict = LinearMap::<_, _, consts::U128>::new();
    for (&from, &to) in FROM.iter().zip(TO.iter()) {
        dict.insert(from, to).ok();
    }

    output.push_str("deviceid=").ok();
    write!(output, "{:08x}{:08x}", hal::deviceid1(), hal::deviceid0()).ok();
    write!(output, " channel={} TxPower=+8dBm app=puzzle.hex\n", rtx.channel()).ok();

    let task = async {
        let mut packet = Packet::new().await;
        stx.write(output.as_bytes());

        loop {
            let crcres = rrx.read(&mut packet).await;
            let len = packet.len();
            let lqi = if len >= 3 {
                Some(packet.lqi())
            } else {
                // packet is too small; LQI is not valid
                None
            };

            let mut busy = false;
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
                        b"correct"
                    } else {
                        b"incorrect"
                    });
                }

                busy = rtx.write(&packet).await.is_err();
            }

            output.clear();
            write!(
                &mut output,
                "received {} byte{}",
                len,
                if len == 1 { "" } else { "s" }
            )
                .ok();

            let (res, crc) = match crcres {
                Ok(x) => ("Ok", x),
                Err(x) => ("Err", x),
            };

            write!(&mut output, " (CRC={}({:#06x})", res, crc).ok();
            if let Some(lqi) = lqi {
                write!(&mut output, ", LQI={}", lqi).ok();
            }
            output.push_str(")\n").ok();

            if busy {
                output.push_str("didn't reply -- channel was busy\n").ok();
                stx.write(output.as_bytes());
            }

            stx.write(output.as_bytes());
        }
    };

    executor::run!(task)
}
