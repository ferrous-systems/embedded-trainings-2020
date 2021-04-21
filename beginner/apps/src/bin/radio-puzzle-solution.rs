#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::{consts, LinearMap, Vec};
use panic_log as _; // the panicking behavior

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    /* # Build a dictionary */
    let mut dict = LinearMap::<u8, u8, consts::U128>::new();
    //                                 ^^^^^^^^^^^^ NOTE larger capacity

    // the IEEE 802.15.4 packet that will carry our data
    let mut packet = Packet::new();
    for plainletter in 0..=127 {
        //             ^^^^^^^ NOTE complete ASCII range
        packet.copy_from_slice(&[plainletter]);

        radio.send(&mut packet);

        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            // response should be one byte large
            if packet.len() == 1 {
                let cipherletter = packet[0];

                // NOTE we want to map in reverse: from cipherletter to plainletter
                dict.insert(cipherletter, plainletter)
                    .expect("dictionary full");
            } else {
                log::error!("response packet was not a single byte");
                dk::exit()
            }
        } else {
            log::error!("no response or response packet was corrupted");
            dk::exit()
        }
    }

    /* # Retrieve the secret string */
    packet.copy_from_slice(&[]); // empty packet
    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        log::error!("no response or response packet was corrupted");
        dk::exit()
    }

    log::info!(
        "ciphertext: {}",
        str::from_utf8(&packet).expect("packet was not valid UTF-8")
    );

    /* # Decrypt the string */
    let mut buffer = Vec::<u8, consts::U128>::new();

    // iterate over the bytes
    for cipherletter in packet.iter() {
        let key = cipherletter;
        let value = dict[key];

        let plainletter = value;
        buffer.push(plainletter).expect("buffer full");
    }

    log::info!(
        "plaintext:  {}",
        str::from_utf8(&buffer).expect("buffer contains non-UTF-8 data")
    );

    /* # Verify decrypted text */
    packet.copy_from_slice(&buffer);

    radio.send(&mut packet);

    if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_err() {
        log::error!("no response or response packet was corrupted");
        dk::exit()
    }

    log::info!(
        "Dongle response: {}",
        str::from_utf8(&packet).expect("response was not UTF-8")
    );

    dk::exit()
}
