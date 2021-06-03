#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::{consts, Vec};
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    let mut packet = Packet::new();

    /* # Retrieve the secret string */
    packet.copy_from_slice(&[]); // empty packet
    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    defmt::info!(
        "ciphertext: {}",
        str::from_utf8(&packet).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string */
    let mut buf = Vec::<u8, consts::U128>::new();

    // iterate over the bytes
    for input in packet.iter() {
        // process each byte
        // here we should do the reverse mapping; instead we'll do a shift for illustrative purposes
        let output = input + 1;
        buf.push(output).expect("buffer full");
    }

    defmt::info!(
        "plaintext: {}",
        str::from_utf8(&buf).expect("buffer contains non-UTF-8 data")
    );

    dk::exit()
}
