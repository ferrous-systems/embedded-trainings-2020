# Tooltips

Besides the ones covered in this workshop, there are many more tools that make embedded development easier.
Here, we'd like to introduce you to some of these tools and encourage you to play around with them and adopt them if you find them helpful!

## `cargo-bloat`

`cargo-bloat` is a useful tool to analyze the binary size of a program. You can install it through cargo:

``` console
$ cargo install cargo-bloat
(..)
Installed package `cargo-bloat v0.10.0` (..)
```

Let's inspect our beginner course's `hello` program with it:

``` console
$ cd beginner/apps
$ cargo bloat --bin hello
File  .text   Size      Crate Name
0.7%  13.5% 1.3KiB        std <char as core::fmt::Debug>::fmt
0.5%   9.6%   928B      hello hello::__cortex_m_rt_main
0.4%   8.4%   804B        std core::str::slice_error_fail
0.4%   8.0%   768B        std core::fmt::Formatter::pad
0.3%   6.4%   614B        std core::fmt::num::<impl core::fmt::Debug for usize>::fmt
(..)
5.1% 100.0% 9.4KiB            .text section size, the file size is 184.5KiB
```

This breaks down the size of the `.text` section by function. This breakdown can be used to identify the largest functions in the program; those could then be modified to make them smaller.

## `cargo-flash`

`cargo-flash` is a tool that flashes a Rust program on a microcontroller.
From within a Cargo project it can be used like this:

``` console
$ # flash the `hello` program on an nRF52840 microcontroller
$ cargo flash --chip nRF52840_xxAA --bin hello
    Flashing target/thumbv7em-none-eabihf/debug/blinky
     Erasing sectors ✔ [00:00:00] [####]  20.00KB/ 20.00KB @  21.24KB/s (eta 0s )
 Programming pages   ✔ [00:00:01] [####]  20.00KB/ 20.00KB @   6.38KB/s (eta 0s )
    Finished in 1.995s
```

The tool will reset the device after flashing it so when it finishes the device will be running the new firmware.
This subcommand will build the program first so you'll always flash the latest version.

The tool can also flash pre-built Rust programs so you could distribute binary releases of your firmware to your users and they can use `cargo-flash` to flash those binaries.

``` console
$ # you
$ cargo build --bin app --release

$ # distribute target/thumb*/release/app to your users

$ # your users
$ cargo flash --chip nRF52840_xxAA --elf ./app
```
