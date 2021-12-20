#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// use apps as _; // this defines the panicking behaviour


#[entry]
fn main() -> ! {
    dk::init().unwrap();

    foo();

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn foo() {
    asm::nop();
    bar();
}

#[inline(never)]
fn bar() {
    let i = index();
    let array = [0, 1, 2];
    let x = array[i]; // out of bounds access

    defmt::info!("{}", x);
}

fn index() -> usize {
    3
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::panic!("lalal {}", info);
}