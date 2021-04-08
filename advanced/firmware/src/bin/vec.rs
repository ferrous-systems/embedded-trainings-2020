#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use heapless::{consts, Vec};
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a stack-allocated `Vec` with capacity for 6 bytes
    let mut buffer = Vec::<u8, consts::U6>::new();
    //                                 ^^ capacity; this is a type not a value

    // `heapless::Vec` exposes the same API surface as `std::Vec` but some of its methods return a
    // `Result` to indicate whether the operation failed due to the `heapless::Vec` being full
    defmt::info!("start: {:?}", defmt::Debug2Format(&buffer));
    //                                 ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log
    //                                                     `heapless` data structures (like `Vec` here)
    //                                                     with `defmt`

    buffer.push(0).expect("buffer full");
    defmt::info!("after `push`: {:?}", defmt::Debug2Format(&buffer));

    buffer.extend_from_slice(&[1, 2, 3]).expect("buffer full");
    defmt::info!("after `extend`: {:?}", defmt::Debug2Format(&buffer));

    // TODO try this operation
    // buffer.extend_from_slice(&[4, 5, 6, 7]).expect("buffer full");

    // TODO try changing the capacity of the `heapless::Vec`

    dk::exit()
}
