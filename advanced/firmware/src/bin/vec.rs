#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use heapless::{consts, Vec};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a stack-allocated `Vec` with capacity for 6 bytes
    let mut buffer = Vec::<u8, consts::U6>::new();
    //                                 ^^ capacity; this is a type not a value

    // `heapless::Vec` exposes the same API surface as `std::Vec` but some of its methods return a
    // `Result` to indicate whether the operation failed due to the `heapless::Vec` being full
    log::info!("start: {:?}", buffer);

    buffer.push(0).expect("buffer full");
    log::info!("after `push`: {:?}", buffer);

    buffer.extend_from_slice(&[1, 2, 3]).expect("buffer full");
    log::info!("after `extend`: {:?}", buffer);

    // TODO try this operation
    // buffer.extend_from_slice(&[4, 5, 6, 7]).expect("buffer full");

    // TODO try changing the capacity of the `heapless::Vec`

    dk::exit()
}
