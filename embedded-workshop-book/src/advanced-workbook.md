# Advanced Workbook

In this workshop you'll learn to:

- work with registers and peripherals from Rust
- handle external events in embedded Rust applications
- debug evented applications
- test `no_std` code

To put these concepts and techniques in practice you'll write a toy USB device application that gets enumerated and configured by the host. This embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

You have received two development boards for this workshop. We'll only use the nRF52840 Development Kit, the larger of the two, in the advanced workshop.

## The nRF52840 Development Kit

The board has two USB ports: J2 and J3 and an on-board J-Link programmer / debugger -- [there are instructions to identify the ports in a previous section][id-ports]. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using both ports.

[id-ports]: ./hardware.md#nrf52840-development-kit-dk

## The nRF52840

Both development boards have an nRF52840 microcontroller. Here are some details about it that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)
