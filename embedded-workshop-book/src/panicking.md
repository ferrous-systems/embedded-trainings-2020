# Panicking

✅ Open the `src/bin/panic.rs` file and click the "Run" button.

This program attempts to index an array beyond its length and this results in a panic.

``` console
(HOST) INFO  flashing program (34.79 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
ERROR:panic_log -- panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:29:13
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
[...]
   7: panic::bar
        at src/bin/panic.rs:29:13
   8: panic::foo
        at src/bin/panic.rs:22:5
   9: panic::__cortex_m_rt_main
        at src/bin/panic.rs:12:5
  10: main
        at src/bin/panic.rs:8:1
[...]
(HOST) ERROR the program panicked
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
