// f6c27c0a5af464795d1a64c45fa1b3039f44a62a
#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::fmt::Write as _;

use hal::{
    radio::{self, Channel, Packet},
    usbd,
};
use heapless::{consts, String};
use panic_abort as _;

#[no_mangle]
fn main() -> ! {
    let mut stx = usbd::serial();
    let (mut rtx, mut rrx) = radio::claim(Channel::_20);
    let mut output = String::<consts::U64>::new();

    output.push_str("deviceid=").ok();
    write!(output, "{:08x}{:08x}", hal::deviceid1(), hal::deviceid0()).ok();
    write!(output, " channel={} TxPower=+8dBm\n", rtx.channel()).ok();

    let task = async {
        let mut packet = Packet::new().await;
        stx.write(output.as_bytes());

        loop {
            let crcres = rrx.read(&mut packet).await;
            let lqi = if packet.len() >= 3 {
                Some(packet.lqi())
            } else {
                // packet is too small; LQI is not valid
                None
            };

            let mut busy = false;
            if crcres.is_ok() {
                // packet.reverse();
                busy = rtx.write(&packet).await.is_err();
            }

            output.clear();
            let len = packet.len();
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
