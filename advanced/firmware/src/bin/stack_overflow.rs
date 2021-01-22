#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_log as _; // panic handler

#[entry]
fn main() -> ! {
    // board initialization
    dk::init().unwrap();

    log::info!("provoking stack overflow...");
    spam();

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn spam() {
    // allocate and initialize one kilobyte of stack memory to provoke stack overflow
    let use_stack = [0xAA_u32; 1024];

    log::info!(
        "address of current `use_stack`: {:?}",
        &use_stack as *const u32
    );

    spam(); // infinite recursion
}
