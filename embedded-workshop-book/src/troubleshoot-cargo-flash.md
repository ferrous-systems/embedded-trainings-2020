# `cargo-flash` is not working

## Debug power request failed

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
 ERROR probe_rs::architecture::arm::communication_interface > Debug power request failed
Error processing command: An error specific to the selected architecture occured
```

This is a spurious error that occurs only once on a new development kit. Running the command again should make the error go away. If you still get the error run `RUST_LOG=probe_rs=debug cargo flash --chip nRF52840_xxAA --bin hello ` once.

## 'erase_sector' failed with code 1

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
(..)
Error failed to flash app: The execution of 'erase_sector' failed with code 1
```

flash write protection is enabled in the device. To disable it use the `nrf-recover` tool. Instructions can be found in the [`nrf-recover` section of the Installation Instructions].

[`nrf-recover` section of the Installation Instructions]: ./installation.md#nrf-recover


## Linux permissions

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
Error: An error specific to a probe type occured: USB error while opening USB device: Access denied (insufficient permissions)

Caused by:
    USB error while opening USB device: Access denied (insufficient permissions)
```

udev rules need to be changed to allow non-root access. Instructions can be found in the [`Linux only: USB` section of the Installation Instructions].

[`Linux only: USB` section of the Installation Instructions]: ./installation.md#linux-only-usb

## Wrong Windows Driver

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
Error: An error specific to a probe type occured: USB error while opening USB device: Entity not found

Caused by:
    USB error while opening USB device: Entity not found
```

You need to bind the BULK interface of the J-Link USB device to the WinUSB driver using the Zadig tool. Instructions can be found in the [`Windows only: Zadig JLink driver` section of the Installation Instructions].

[`Windows only: Zadig JLink driver` section of the Installation Instructions]: ./installation.md#windows-only-zadig-jlink-driver
