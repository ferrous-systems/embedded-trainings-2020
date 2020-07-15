# Panicking

✅ Open the `src/bin/panic.rs` file and click the "Run" button.

This program attempts to index an array beyond its length and this results in a panic.

``` console
ERROR:panic_log -- panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:29:13
stack backtrace:
   0: 0x000022f0 - __bkpt
   1: 0x00002010 - rust_begin_unwind
   2: 0x00000338 - core::panicking::panic_fmt
   3: 0x00000216 - core::panicking::panic_bounds_check
   4: 0x0000016a - panic::bar
   5: 0x00000158 - panic::foo
   6: 0x00000192 - panic::__cortex_m_rt_main
   7: 0x00000178 - main
   8: 0x0000199e - Reset
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `panic_log` crate but we can also implement it manually: 

✅ Comment out the `panic_log` import and add the following function to the example:

``` rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    loop {
        asm::bkpt()
    }
}
```

Now run the program again. Try changing the format string of the `error!` macro.
