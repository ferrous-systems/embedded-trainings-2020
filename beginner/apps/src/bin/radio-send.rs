#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use dk::ieee802154::{Packet, Channel, TxPower};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;

    // these are the initial settings
    radio.set_channel(Channel::_20);
    radio.set_txpower(TxPower::Pos8dBm);

    let mut packet = Packet::new();
    packet.copy_from_slice(b"Hello");
    //radio.send(&packet);
    let res = radio.try_send(&packet);
    log::info!("{:?}", res);

    loop {
        asm::bkpt();
    }
}
