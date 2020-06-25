#![deny(unused_must_use)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// NOTE you can use `FnvIndexMap` instead of `LinearMap`; the former may have better
// lookup performance when the dictionary contains a large number of items but performance is
// not important for this exercise
use heapless::{consts, LinearMap};
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    // a dictionary with capacity for 2 elements
    let mut dict = LinearMap::<_, _, consts::U2>::new();
    //                                       ^^ capacity; this is a type not a value

    // do some insertions
    dict.insert(b'A', b'*').expect("dictionary full");
    dict.insert(b'B', b'/').expect("dictionary full");

    // do some lookups
    let key = b'A';
    let value = dict[&key]; // the key needs to be passed by reference

    log::info!("{} -> {}", key, value);
    // more readable
    log::info!("{:?} -> {:?}", key as char, value as char);

    // TODO try another insertion
    // TODO try looking up a key not contained in the dictionary
    // TODO try changing the capacity

    dk::exit()
}
