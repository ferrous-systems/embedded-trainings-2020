# Listing USB Devices

In the `tools` folder you'll find `usb-list`: a minimal cross-platform version of the `lsusb` tool. 

✅ To list all USB devices, run the progam using `cargo run` from `tools/usb-list`.


``` console
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
```

The goal of this exercise is to get the nRF52840 SoC to show in this list. The embedded application will use the vendor ID (VID) and product ID (PID) defined in `advanced/common/consts`; the `usb-list` tool will highlight the USB device that matches that VID PID pair.

``` console
$ # expected output
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 059: ID 2020:0717 <- nRF52840 on the nRF52840 Development Kit
```