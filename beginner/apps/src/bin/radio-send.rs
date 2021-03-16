#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet, TxPower};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;

    // these are the default settings of the DK's radio
    // NOTE if you ran `change-channel` then you may need to update the channel here
    radio.set_channel(Channel::_20); // <- must match the Dongle's listening channel
    radio.set_txpower(TxPower::Pos8dBm);

    let mut packet = Packet::new();

    // these three are equivalent
    let msg: &[u8; 5] = &[72, 101, 108, 108, 111];
    // let msg: &[u8; 5] = &[b'H', b'e', b'l', b'l', b'o'];
    // let msg: &[u8; 5] = b"Hello";

    log::info!(
        "sending: {}",
        str::from_utf8(msg).expect("msg is not valid UTF-8 data")
    );

    packet.copy_from_slice(msg);

    radio.send(&mut packet);

    dk::exit();
}
