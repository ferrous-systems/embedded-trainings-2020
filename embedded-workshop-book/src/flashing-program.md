# Flashing the Program

✅ Use the following command to flash the program to the device.

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
```

> NOTE: If you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear.

This subcommand will build the program first so you'll always flash the latest version.

The `cargo-flash` subcommand flashes and runs the program but won't display logs. It is a deployment tool.

**🔎 How does flashing work?**

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, named J-Link, is connected to the nRF52840 through a electrical interface known as SWD. The SWD protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.
