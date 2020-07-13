#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::{fmt::Write as _, convert::TryFrom};

use async_core::unsync::Mutex;
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

    let stx = Mutex::new(usbd::serial());
    let (mut hidout, _) = usbd::hid();
    let (rtx, mut rrx) = radio::claim(Channel::_25);
    let mut output = String::<consts::U128>::new();

    let mut dict = LinearMap::<_, _, consts::U128>::new();
    for (&from, &to) in FROM.iter().zip(TO.iter()) {
        dict.insert(from, to).ok();
    }

    output.push_str("deviceid=").ok();
    write!(output, "{:08x}{:08x}", hal::deviceid1(), hal::deviceid0()).ok();
    write!(output, " channel={} TxPower=+8dBm app=puzzle.hex\n", rtx.channel()).ok();

    let rtx = Mutex::new(rtx);

    let t1 = async {
        let mut output = String::<consts::U128>::new();
        let mut hidbuf = usbd::Packet::new().await;
        let zlp = radio::Packet::new().await;

        loop {
            hidout.recv(&mut hidbuf).await;
            semidap::info!("HID: {}", *hidbuf);

            let arg = if hidbuf.len() == 1 {
                // Linux / macOS
                Some(hidbuf[0])
            } else if hidbuf.len() == 64 {
                // Windows (it zero pads the packet)
                Some(hidbuf[0])
            } else {
                None
            };

            if let Some(arg) = arg {
                if let Ok(chan) = Channel::try_from(arg) {
                    let mut rtx = rtx.lock().await;
                    rtx.set_channel(chan);
                    // send a zero-length packet to force the radio to listen on the new channel
                    rtx.write(&zlp).await.ok();
                    drop(rtx);

                    output.clear();
                    writeln!(output, "now listening on channel {}", chan).ok();
                    stx.lock().await.write(output.as_bytes());
                } else {
                    stx.lock()
                        .await
                        .write(b"requested channel is out of range (11-26)\n");
                }
            } else {
                stx.lock().await.write(b"invalid HID packet\n");
            }
        }
    };

    let t2 = async {
        let mut packet = Packet::new().await;
        stx.lock().await.write(output.as_bytes());

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

                busy = rtx.lock().await.write(&packet).await.is_err();
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
                stx.lock().await.write(output.as_bytes());
            }

            stx.lock().await.write(output.as_bytes());
        }
    };

    executor::run!(t1, t2)
}
