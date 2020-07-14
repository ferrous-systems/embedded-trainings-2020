#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::{convert::TryFrom, fmt::Write as _};

use async_core::unsync::Mutex;
use hal::{
    radio::{self, Channel},
    usbd,
};
use heapless::{consts, String};
use panic_abort as _;

#[no_mangle]
fn main() -> ! {
    let stx = Mutex::new(usbd::serial());
    let (mut hidout, _) = usbd::hid();
    let (rtx, mut rrx) = radio::claim(Channel::_20);

    let mut output = String::<consts::U128>::new();

    output.push_str("deviceid=").ok();
    write!(output, "{:08x}{:08x}", hal::deviceid1(), hal::deviceid0()).ok();
    write!(
        output,
        " channel={} TxPower=+8dBm app=loopback.hex\n",
        rtx.channel()
    )
    .ok();

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
        let mut packet = radio::Packet::new().await;
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
                packet.reverse();
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
