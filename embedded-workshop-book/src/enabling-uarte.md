# Enabling the UARTE0 peripheral

Write a simple program which uses the PAC to enable the UARTE0. See how writing arbitrary values to the ENABLE field in the ENABLE register is unsafe, because only values 0 or 8 should be used.

## In this exercise you will learn how to:
* to safely write into a register 
* how to write raw bits into a register
* how to read a register

## Prerequisites
* basic use of closures
* usage of the svd2rust's read/write/modify API

## Tasks
* Find out which values can be written into the `enable` register.
* Take ownership of the board's peripherals
* Write a helper function that reads `UARTE0`'s `enable` register and prints the enable status.
* Enable the UARTE0 peripheral using a safe method.
* Disable the UARTE0 peripheral by writing raw bits in it (unsafe).

Final terminal output:

```terminal
Uarte0 is disabled.
Uarte0 is ensabled.
Uarte0 is disabled.
```

Find the starter code in `down-the-stack/apps/src/bin/uarte_enable.rs`
Find the full solution in `down-the-stack/solutions/src/bin/uarte_enable.rs`


## Step-by-Step Solution

### Step 1: Find the values that can be written in the `enable` register:

0: disabled
8: enabled

### Step 2: Import the PAC

In the Cargo configuration file for the `apps` package, `down-the-stack/apps/Cargo.toml`, add:

```
dk_pac = { path = "../dk_pac", features = ["critical-section"]}
```
In the main source file for the `uarte_enable` binary, `apps/bin/uarte_enable.rs`, add:

```rust 
use dk_pac::UARTE0;
```

### Step 3: Take ownership of the peripherals with `take()`

Take ownership of the peripherals with `take()`. Take note, the take returns an `Option<T>` so that needs to be taken care of. Bind the `UARTE0` peripheral to it's own variable.

```rust
let periph = dk_pac::Peripherals::take().unwrap();
let uarte = periph.UARTE0;
```

### Step 4: Write a helper function to get the status of the register.

The helper function either reads the raw bits of the enable register or makes use of the specific method available. 

The function prints "Uarte0 is enabled" or "Uarte0 is disabled" depending on the case. Add a function call to `fn main()`.

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

### Step 5: Enable the peripheral safely.

Enable the peripheral safely by passing `w.enable().enabled()` in the closure of `write()`. Call the helper function after this new line and run your code. 

It should print:

```terminal
Uarte0 is disabled.
Uarte0 is ensabled.
```

```rust
uarte.enable.write(|w| w.enable().enabled());
```

### Step 6: Disable the peripheral unsafely by writing raw bits into the register. 

Write 0 into the register to disable it by passing the closure `|w| w.bits(0x00 as u32)` to the `write()` method. This action is unsafe, so it needs to be in an unsafe block. Call the helper function once more and run your code. Compare your output with the expected output on top of this page. 

```rust
unsafe {
        uarte.enable.write(|w| w.bits(0x00 as u32));
    }
```
