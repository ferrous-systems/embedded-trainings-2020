# Hardware

In this workshop we'll use both the nRF52840 Development Kit (DK) and the nRF52840 Dongle. We'll mainly develop programs for the DK and use the Dongle to assist with some exercises.

For the span of this workshop keep the nRF52840 DK connected to your PC using a micro-USB cable. Connect the USB cable to the J2 port on the nRF52840 DK. Instructions to identify the USB ports on the nRF52840 board can be found in the top level README file.

## The nRF52840

Some details about the nRF52840 microcontroller that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

[❗️Info about Dongle]