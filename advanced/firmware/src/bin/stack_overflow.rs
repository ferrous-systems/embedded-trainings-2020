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
    spam(0);

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn spam(n: u32) {
    // allocate and initialize 4 kilobytes of stack memory to provoke stack overflow
    let use_stack = [n; 1024];

    log::info!(
        "address of current `use_stack` at recursion depth {:?}: {:?}",
        use_stack[1023], // "use" use_stack to prevent it from being optimized out
        &use_stack as *const u32
    );

    let next = n + 1;
    spam(next); // infinite recursion
}
