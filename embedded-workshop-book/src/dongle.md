# nRF52840 Dongle

From this section on, we'll use the nRF52840 Dongle in addition to the nRF52840 DK. We'll run some pre-compiled programs on the Dongle and write programs for the DK that will interact with the Dongle over a radio link.

Install the `dongle-flash` tool by running the following command from the `tools/dongle-flash` directory.

``` console
$ cargo install --path . -f
```

The Dongle does not contain an on-board debugger, like the DK, so we cannot use `probe-rs` tools to write programs into it. Instead, the Dongle's stock firmware comes with a *bootloader*.

When put in bootloader mode the Dongle will run a bootloader program instead of the last application that was flashed into it. This bootloader program will make the Dongle show up as a USB CDC ACM device (AKA Serial over USB device) that accepts new application images over this interface. We'll use the `nrfutil` tool to communicate with the bootloader-mode Dongle and flash new images into it.

To put the Dongle in bootloader mode connect it to your laptop / PC  / mac and then press its *reset* button. The Dongle has two buttons: a round-ish user button (SW1) and a square-ish reset button (RESET); the latter is mounted "sideways". The buttons are next to each other. The RESET button is mounted closer to the edge of the board that has the Nordic logo on silkscreen and the actual button is facing towards that edge. The opposite edge of the board is narrower and has the surface USB connector; this is the end that goes into your PC USB port.

When the Dongle is in bootloader mode its red LED will oscillate in intensity. Alternatively, the status can be checked using the `usb-list` tool introduced below. The Dongle will also appear as a USB CDC ACM device with vendor ID `0x1915` and product ID `0x521f`.

In the `tools` folder you'll find `usb-list`: a minimal cross-platform version of the `lsusb` tool. Run it (`cargo run` from `tools/usb-list`) to list all USB devices; the Dongle will be highlighted in the output, along with a note if in bootloader mode.


``` console
$ cargo run
(..)
Bus 001 Device 016: ID 1915:521f <- nRF52840 Dongle (in bootloader mode)
```

Now that the device is in bootloader mode browse to the `boards/dongle` directory. You'll find some `*.hex` files there. These are pre-compiled Rust programs that have been converted into the Intel Hex format that the `nrfutil` tool expects.

For the next section you'll need to flash the `loopback.hex` file into the Dongle. There are two ways to do this. You can make 2 long `nrfutil` invocations or you can use our `dongle-flash` tool, which will invoke `nrfutil` for you. The `dongle-flash` way is shown below:

``` console
$ dongle-flash loopback.hex
packaging iHex using nrfutil ...
DONE
  [####################################]  100%
Device programmed.
```

After the device has been programmed it will automatically reset and start running the new application.

The `loopback` application will *blink* the red LED in a heartbeat fashion: two fast blinks (LED on then off) followed by two periods of silence (LED off). The application will also make the Dongle enumerate itself as a CDC ACM device. If you run  `usb-list` tool (from the `tools/usb-list` directory) you should see the newly enumerated Dongle in the output:

``` console
$ cargo run
Bus 001 Device 020: ID 2020:0309 <- nRF52840 Dongle (loopback.hex)
```

The `loopback` app will log messages over the USB interface. To display these messages on the host we have provided a cross-platform tool: `serial-term`. Install it by running the following command from the `tools/serial-term` directory.

``` console
$ cargo install --path . -f
```

If you run the `serial-term` application you should see the following output:

``` console
$ serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm app=loopback.hex
(..)
```

This line is printed by the `loopback` app on boot. It contains the device ID of the dongle, a 64-bit unique identifier (so everyone will see a different number); the radio channel that the device will use to communicate; and the transmission power of the radio in dBm.

Leave the Dongle connected and the `serial-term` application running. Now we'll switch back to the Development Kit.
