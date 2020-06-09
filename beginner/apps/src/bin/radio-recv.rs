#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m::asm;
use cortex_m_rt::entry;
use dk::ieee802154::Packet;
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    // initializes the peripherals
    let board = dk::init().unwrap();
    let mut radio = board.radio;

    let mut packet = Packet::new();
    let msg = b"olleh";
    packet.copy_from_slice(msg);
    radio.send(&packet);
    log::info!("sent: {:?}", msg);
    let crc = radio.recv(&mut packet);
    let s = str::from_utf8(&*packet).expect("response is not valid UTF-8");
    log::info!("received: {} (CRC={:?})", s, crc);

    loop {
        asm::bkpt();
    }
}
