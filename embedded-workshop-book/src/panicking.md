# Panicking

✅ Open the `src/bin/panic.rs` file and click the "Run" button.

This program attempts to index an array beyond its length and this results in a panic.

``` console
────────────────────────────────────────────────────────────────────────────────
ERROR panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:32:13
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
   1: lib::inline::__udf
        at ./asm/inline.rs:172:5
   2: __udf
        at ./asm/lib.rs:49:17
   3: cortex_m::asm::udf
        at /Users/name/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.3/src/asm.rs:43:5
   4: rust_begin_unwind
        at /Users/name/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.3.0/src/lib.rs:72:9
   5: core::panicking::panic_fmt
        at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
   6: core::panicking::panic_bounds_check
        at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:76:5
   7: panic::bar
        at src/bin/panic.rs:32:13
   8: panic::foo
        at src/bin/panic.rs:25:5
   9: panic::__cortex_m_rt_main
        at src/bin/panic.rs:15:5
  10: main
        at src/bin/panic.rs:11:1
  11: Reset
(HOST) ERROR the program panicked
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `panic_log` crate but we can also implement it manually: 

✅ Comment out the `panic_probe` import and add the following function to the example:

``` rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::panic!("{}", info);
}
```

Now run the program again. Try changing the format string of the `panic!` macro.
