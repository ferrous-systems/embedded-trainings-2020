#![deny(unused_must_use)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use heapless::{consts, Vec};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a buffer with capacity for 2 bytes
    let mut buffer = Vec::<u8, consts::U2>::new();
    //                               ^^ capacity; this is a type not a value

    // do some insertions
    buffer.push(b'H').expect("buffer full");
    buffer.push(b'i').expect("buffer full");

    // look into the contents so far
    log::info!("{:?}", buffer);
    // or more readable
    // NOTE as long as you only push bytes in the ASCII range (0..=127) the conversion should work
    log::info!(
        "{}",
        str::from_utf8(&buffer).expect("content was not UTF-8")
    );

    // TODO try another insertion
    // TODO try changing the capacity

    dk::exit()
}
