#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Error, Packet};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    // initializes the peripherals
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    let mut packet = Packet::new();
    let msg = b"olleh";
    packet.copy_from_slice(msg);
    radio.send(&packet);
    log::info!("sent: {:?}", msg);
    let timeout = 1_000;
    let res = radio.recv_timeout(&mut packet, &mut timer, timeout);

    match res {
        Ok(crc) => {
            log::info!(
                "received: {} (CRC={})",
                str::from_utf8(&*packet).expect("response is not valid UTF-8"),
                crc
            );
        }
        Err(Error::Crc(crc)) => log::error!("invalid CRC: {:06x}", crc),
        Err(Error::Timeout) => log::error!("no response within {} ms", timeout),
    }

    dk::exit()
}
