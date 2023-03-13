# Enabling the UARTE0 peripheral

Write a simple program which uses the PAC to enable the UART. See how writing arbitrary values to the ENABLE field in the ENABLE register is unsafe, because only values 0 or 8 should be used.

## In this exercise you will learn how to:
* to safely write into a register 
* how to unsafely write into a register
* how to read a register

## Prerequisites
* basic use of closures
* usage of the svd2rust's read/write/modify API

## Tasks
* Find out which values can be written into the `enable` register.
* Take ownership of the boards peripherals
* Write a helper function that reads the UARTE0's `enable` register and print's status.
* Enable the UARTE0 peripheral using a safe method.
* Disable the UARTE0 peripheral by writing raw bits in it (unsafe).

Final terminal output:

```terminal
Uarte0 is disabled.
Uarte0 is ensabled.
Uarte0 is disabled.
```

## Step-by-Step Solution

✅ Find the values that can be written in the `enable` register:

0: disabled
8: enabled

✅ Import the PAC

In `down-the-stack/apps/Cargo.toml`:

```
dk_pac = { path = "../dk_pac", features = ["critical-section"]}
```
In `apps/bin/uarte_enable.rs`:

```rust 
use dk_pac::UARTE0;
```

✅ Take ownership of the peripherals with `take()` and bind the `UARTE0` peripheral to it's own variable

```rust
let periph = dk_pac::Peripherals::take().unwrap();
let uarte = periph.UARTE0;
```

✅ Write a helper function that reads the bits of the enable register. It prints "Uarte0 is enabled", when the value is not 0. If it is 0, it prints "Uarte0 is disabled". Add a function call to `fn main()`

Run the code. The terminal output should read: "Uarte0 is disabled".

```rust
fn is_uarte_enabled(uarte: &UARTE0) {
    if uarte.enable.read().enable().is_enabled() {
        defmt::println!("Uarte0 is enabled");
    } else {
        defmt::println!("Uarte0 is disabled");
    }
}
```

✅ Enable the peripheral safely by passing `w.enable().enabled()` in the closure. Call the helper function after this new line and run your code. 

It should print:

```terminal
Uarte0 is disabled.
Uarte0 is ensabled.
```

```rust
uarte.enable.write(|w| w.enable().enabled());
```

✅ Disable the peripheral unsafely by writing raw bits into the register. 

```rust
unsafe {
        uarte.enable.write(|w| w.bits(0x00 as u32));
    }
```
