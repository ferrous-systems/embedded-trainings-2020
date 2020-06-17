#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    let mut packet = Packet::new();

    // try these
    let msg = "";
    // let msg = "A";
    // let msg = "Hello?";

    packet.copy_from_slice(msg.as_bytes());
    radio.send(&packet);
    log::info!("sent: {:?}", msg);
    radio.recv(&mut packet).ok();
    if let Ok(s) = str::from_utf8(&*packet) {
        log::info!("received: {}", s);
    } else {
        log::info!("received: {:?}", &*packet);
    }

    dk::exit()
}
