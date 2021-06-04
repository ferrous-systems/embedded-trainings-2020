#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::{LinearMap, Vec};
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

    /* # Build a dictionary */
    let dict = LinearMap::<u8, u8, 128>::new();

    let mut packet = Packet::new();
    for source in 0..=127 {
        packet.copy_from_slice(&[source]);

        radio.send(&mut packet);

        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            // response should be one byte large
            if packet.len() == 1 {
                let _destination = packet[0];

            // TODO insert the key-value pair
            // dict.insert(/* ? */, /* ? */).expect("dictionary full");
            } else {
                defmt::error!("response packet was not a single byte");
                dk::exit()
            }
        } else {
            defmt::error!("no response or response packet was corrupted");
            dk::exit()
        }
    }

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
    let mut buffer = Vec::<u8, 128>::new();

    // iterate over the bytes
    for byte in packet.iter() {
        // NOTE this should map from the encrypted letter to the plaintext letter
        let key = byte;
        let value = dict[key];
        buffer.push(value).expect("buffer full");
    }

    defmt::info!(
        "plaintext: {}",
        str::from_utf8(&buffer).expect("buffer contains non-UTF-8 data")
    );

    /* # (NEW) Verify decrypted text */
    packet.copy_from_slice(&buffer);

    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        defmt::error!("no response or response packet was corrupted");
        dk::exit()
    }

    defmt::info!(
        "Dongle response: {}",
        str::from_utf8(&packet).expect("response was not UTF-8")
    );

    dk::exit()
}
