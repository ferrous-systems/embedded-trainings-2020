#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use panic_log as _; // the panicking behavior

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    let mut packet = Packet::new();

    // try these
    let msg = b"";
    // let msg = b"A";
    // let msg = b"Hello?";

    packet.copy_from_slice(msg);
    log::info!(
        "sending: {}",
        str::from_utf8(msg).expect("msg was not valid UTF-8 data")
    );

    radio.send(&packet);
    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
        log::info!(
            "received: {}",
            str::from_utf8(&packet).expect("response was not valid UTF-8 data")
        );
    } else {
        log::error!("no response or response packet was corrupted");
    }
    dk::exit()
}
