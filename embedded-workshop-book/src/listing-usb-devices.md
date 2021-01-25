# Listing USB Devices

✅ To list all USB devices, run `cargo xtask usb-list` from the `advanced` folder.

❗️ If you haven't yet installed `usb-list`; [installation instructions can be found in a previous section][install].

[install]: ./tooling-check.md#more-tools

``` console
$ cargo xtask usb-list
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 010: ID 1366:1015 <- J-Link on the nRF52840 Development Kit
```

The goal of this workshop is to get the nRF52840 SoC to show in this list. The embedded application will use the vendor ID (VID) and product ID (PID) defined in `advanced/common/consts`; the `usb-list` tool will highlight the USB device that matches that VID PID pair.

``` console
$ # expected output
$ cargo xtask usb-list
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 010: ID 1366:1015 <- J-Link on the nRF52840 Development Kit
Bus 001 Device 059: ID 2020:0717 <- nRF52840 on the nRF52840 Development Kit
```
