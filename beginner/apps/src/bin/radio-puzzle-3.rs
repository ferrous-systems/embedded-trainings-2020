#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use dk::ieee802154::{Channel, Packet};
use heapless::{consts, LinearMap};
use panic_log as _; // the panicking behavior

const TEN_MS: u32 = 10_000;

#[entry]
fn main() -> ! {
    let board = dk::init().unwrap();
    let mut radio = board.radio;
    let mut timer = board.timer;

    // puzzle.hex uses channel 25
    radio.set_channel(Channel::_25);

    // TODO increase capacity
    let dict = LinearMap::<u8, u8, consts::U2>::new();

    let mut packet = Packet::new();
    // TODO do the whole ASCII range [0, 127]
    for source in b'A'..=b'B' {
        packet.copy_from_slice(&[source]);

        radio.send(&packet);

        if radio.recv_timeout(&mut packet, &mut timer, TEN_MS).is_ok() {
            // response should be one byte large
            if packet.len() == 1 {
                let _destination = packet[0];

            // TODO insert the key-value pair
            // dict.insert(/* ? */, /* ? */).expect("dictionary full");
            } else {
                log::error!("response packet was not a single byte");
                dk::exit()
            }
        } else {
            log::error!("no response or response packet was corrupted");
            dk::exit()
        }
    }

    log::info!("{:?}", dict);

    dk::exit()
}
