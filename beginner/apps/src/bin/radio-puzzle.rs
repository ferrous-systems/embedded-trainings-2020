#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::Packet;
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;

    let mut packet = Packet::new();

    // try these
    let msg = "";
    // let msg = "A";
    // let msg = "Hello?";

    packet.copy_from_slice(msg.as_bytes());
    radio.send(&packet);
    log::info!("sent: {:?}", msg);
    // listen for a response packet and ensure it is not corrupted
    if radio.recv(&mut packet).is_ok() {
        // convert the packet contents to str or print error message on failure
        let response = str::from_utf8(&*packet).expect("could not convert response to str");
        log::info!("received: {}", response);
    }
    dk::exit()
}
