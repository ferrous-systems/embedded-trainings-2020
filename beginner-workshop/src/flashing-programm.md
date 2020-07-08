# Flashing the Program

The following command will flash the ELF file to the device.

``` console
$ cargo flash --chip nRF52840_xxAA --elf target/thumbv7em-none-eabi/debug/hello
```

> NOTE if you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear

Alternatively you can run this command, which builds the application before flashing it.

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
```

The `cargo-flash` subcommand flashes and runs the program but won't display logs. It is a deployment tool.

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, named J-Link, is connected to the nRF52840 through a electrical interface known as JTAG. The JTAG protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.
