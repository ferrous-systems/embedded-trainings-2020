# Advanced Workbook

# `advanced`

> Advanced workshop

In this workshop we'll build a toy USB device application that gets enumerated and configured by the host.

The embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

## The nRF52840

Some details about the nRF52840 microcontroller that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

## The nRF52840 Development Kit

The development board we'll use has two USB ports: J2 and J3 -- you can find a description of the board in the top-level README of this repository -- and an on-board J-Link programmer / debugger. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using both ports.
