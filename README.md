# `embedded-trainings-2020`

Material for the beginner and advanced workshops of Oxidize Global (15.07.2020).

> TODO add license text (from commit 1?)

## Required hardware

- [nRF52840 Development Kit (DK)](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK)
- [nRF52840 Dongle](https://www.nordicsemi.com/Software-and-tools/Development-Kits/nRF52840-Dongle)
- 2 micro-USB cables
- 2 available USB-A ports on your laptop / PC (you can use a USB hub if you don't have enough ports)

## Checking the hardware

> NOTE To be done *before* the workshop

### nRF52840 Dongle

Connect the Dongle to your PC/laptop. Its red LED should start oscillating in intensity. The device will also show up as:

**Windows**: a USB Serial Device (COM port) in the Device Manager under the Ports section

**Linux**: a USB device under `lsusb`. The device will have a VID of `0x1915` and a PID of `0x521f` -- the `0x` prefix will be omitted in the output of `lsusb`:

``` console
$ lsusb
(..)
Bus 001 Device 023: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub
```

The device will also show up in the `/dev` directory as a `ttyACM` device:

``` console
$ ls /dev/ttyACM*
/dev/ttyACM0
```

**macOS**: a usb device when executing `ioreg -p IOUSB -b -n "Open DFU Bootloader"`. The device will have a vendor ID (`"idVendor"`) of `6421` and a product ID (`"idProduct"`) of `21023`:

``` console
$ ioreg -p IOUSB -b -n "Open DFU Bootloader"
(...)
| +-o Open DFU Bootloader@14300000  <class AppleUSBDevice, id 0x100005d5b, registered, matched, ac$
  |     {
  |       (...)
  |       "idProduct" = 21023
  |       (...)
  |       "USB Product Name" = "Open DFU Bootloader"
  |       (...)
  |       "USB Vendor Name" = "Nordic Semiconductor"
  |       "idVendor" = 6421
  |       (...)
  |       USB Serial Number" = "CA1781C8A1EE"
  |       (...)
  |     }
  |
```

The device will show up in the `/dev` directory as `tty.usbmodem<USB Serial Number>`:

``` console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodemCA1781C8A1EE1
```

### nRF52840 Development Kit (DK)

Connect one end of a micro USB cable to the USB connector *J2* of the board and the other end to your PC. After connecting the DK to your PC/laptop it will show up as:

**Windows**: a removable USB flash drive (named JLINK) and also as a USB Serial Device (COM port) in the Device Manager under the Ports section

**Linux**: a USB device under `lsusb`. The device will have a VID of `0x1366` and a PID of `0x1015`  -- the `0x` prefix will be omitted in the output of `lsusb`:

``` console
$ lsusb
(..)
Bus 001 Device 014: ID 1366:1015 SEGGER 4-Port USB 2.0 Hub
```

The device will also show up in the `/dev` directory as a `ttyACM` device:

``` console
$ ls /dev/ttyACM*
/dev/ttyACM0
```

**macOS**: a removable USB flash drive (named JLINK) in Finder and also a USB device named "J-Link" when executing `ioreg -p IOUSB -b -n "J-Link"`.

``` console
$ ioreg -p IOUSB -b -n "J-Link"
(...)
  | +-o J-Link@14300000  <class AppleUSBDevice, id 0x10000606a, registered, matched, active, busy 0 $
  |     {
  |       (...)
  |       "idProduct" = 4117
  |       (...)
  |       "USB Product Name" = "J-Link"
  |       (...)
  |       "USB Vendor Name" = "SEGGER"
  |       "idVendor" = 4966
  |       (...)
  |       "USB Serial Number" = "000683420803"
  |       (...)
  |     }
  |
```

The device will also show up in the `/dev` directory as `tty.usbmodem<USB Serial Number>`:

``` console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodem0006834208031
```

The board has several switches to configure its behavior. The out of the box configuration is the one we want. If the above instructions didn't work for you, check the position of the on-board switches:

NOTE: Directions assume you are holding the board "horizontally" with components (switches and button) facing up. In this horizontal position you'll find one USB connector (J2) on the left edge, another USB connector (J3) on the bottom edge and 4 buttons on the bottom right corner.

- Switch SW8, on the bottom edge left corner, is set to the ON position; this is the left position of the two possible positions
- Switch SW9, to the right the left edge USB connector (J2), is set to the VDD position; this is the center position of the three possible positions

## Installation instructions

> NOTE To be done *before* the workshop

### Base Rust installation

Go to https://rustup.rs and follow the instructions.

**Windows**: *Do* install the optional components of the C++ build tools package. The installation size may take up to 2 GB of disk space.

### VS Code

**Windows**: Go to https://code.visualstudio.com and run the installer

**Linux**: Check your Linux distribution package manager (example below). If it's not there follow the instructions on https://code.visualstudio.com/docs/setup/linux

``` console
$ # Arch Linux
$ sudo pacman -S code
```

**macOS**: Go to https://code.visualstudio.com and click on "Download for Mac"

### Rust Analyzer

**All**: Open VS Code and look for Rust Analyzer in the marketplace (bottom icon in the left panel). Then install it.

**Windows**: it's OK to ignore the message about `git` not being installed, if you get one

### Rust Cross compilation support

**All**: Run this command in a terminal:

``` console
$ rustup +stable target add thumbv7em-none-eabi
```

### ELF analysis tools

**All**: Run these commands in a terminal:

``` console
$ rustup +stable component add llvm-tools-preview

$ cargo install cargo-binutils
```

### Python

**Windows**: Go to https://www.python.org/downloads/ and run the Python *3* installer

- in the installer check the "add Python 3.x to PATH" box
- also run the "Disable path length limit" action at the end, if you are on Windows 10 and the option is displayed to you

**Linux**: Install `pip` using the package manager; this will also install Python.

``` console
$ # Arch Linux
$ sudo pacman -S python-pip
```

**macOS**:
Ensure that you have python 3 and pip installed. Refer to the following link for [Instructions on how to install python 3 and pip](https://docs.python-guide.org/starting/install3/osx/)

```console
$ python --version
Python 3.7.7
$ pip --version
pip 20.0.2 from /usr/local/lib/python3.7/site-packages/pip (python 3.7)
```

### nrfutil

**All**: Open a terminal and install nrfutil as follows. *If you are familiar with python, it is advised to do this in a [virtual environment](https://docs.python.org/3/library/venv.html).*

``` console
$ pip install nrfutil
(..)

$ nrfutil version
nrfutil version 6.1.0
```

### USB permissions (Linux only)

1. (Optional) Connect the dongle and check its permissions with these commands:

``` console
$ lsusb -d 1915:521f
Bus 001 Device 016: ID 1915:521f Nordic Semiconductor ASA USB Billboard
$ #   ^         ^^

$ # take note of the bus and device numbers that appear for you when run the next command
$ ls -l /dev/bus/usb/001/016
crw-rw-r-- 1 root root 189, 15 May 20 12:00 /dev/bus/usb/001/016
```

The `root root` part in `crw-rw-r-- 1 root root` indicates the device can only be accessed by the `root` user.

2. Create the following file with the displayed contents. You'll need root permissions to create the file.

``` console
$ cat /dev/udev/rules.d/50-oxidize-global.conf
# udev rules to allow access to USB devices as a non-root user

# nRF52840 Dongle in bootloader mode
ATTRS{idVendor}=="1915", ATTRS{idProduct}=="521f", TAG+="uaccess"

# nRF52840 Dongle applications
ATTRS{idVendor}=="2020", TAG+="uaccess"

# nRF52840 Development Kit
ATTRS{idVendor}=="1366", ATTRS{idProduct}=="1015", TAG+="uaccess"
```

3. Run the following command to make the new udev rules effective

``` console
$ sudo udevadm control --reload-rules
```

4. (Optional) Disconnect and reconnect the dongle. Then check its permissions again.

``` console
$ lsusb
Bus 001 Device 017: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub

$ ls -l /dev/bus/usb/001/017
crw-rw-r--+ 1 root root 189, 16 May 20 12:11 /dev/bus/usb/001/017
```

The `+` part in `crw-rw-r--+` indicates the device can be accessed without `root` permissions.

### JLink driver (Windows only)

On Windows you'll need to associate the nRF52840 Development Kit's USB device to the WinUSB driver.

To do that connect the nRF52840 DK to your PC using micro-USB port J2 (as done before) then download and run the [Zadig] tool.

[Zadig]: https://zadig.akeo.ie/

In Zadig's graphical user interface,

1. Select the 'List all devices' option from the Options menu at the top.

2. From the device (top) drop down menu select "BULK interface (Interface 2)"

3. Once that device is selected, `1366 1015` should be displayed in the USB ID field. That's the Vendor ID - Product ID pair.

4. Select 'WinUSB' as the target driver (right side)

5. Click "Install WinUSB driver". The process may take a few minutes to complete.

### `nrf-recover`

> NOTE(japaric) this may not be necessary in the future -- the probe-rs library may do this on its own

Some nRF52840 devices, specially older revisions, may have parts of their Flash memory locked. To unlock the memory use the `nrf-recover` tool.

This is only relevant to the nRF52840 Development Kit. First connect the nRF52840 DK to your PC using micro-USB J2 (as done before) then run the following commands:

``` console
$ cargo install nrf-recover

$ nrf-recover -y
Starting mass erase...
Mass erase completed, chip unlocked
```

### Cargo subcommands

Install the `cargo-flash` and `cargo-embed` subcommands using the following command:

``` console
$ cargo install cargo-flash cargo-embed cargo-binutils
(..)
     Summary Successfully installed cargo-flash, cargo-embed, cargo-binutils!
```

### "power debug error"

> NOTE(japaric) we need to investigate what this error is about -- it seems to occur only once?

Install `probe-rs-cli`:

``` console
$ cargo install probe-rs-cli
```

Connect the nRF52840 DK to your PC / laptop; use USB connector J2 on the DK. Then run this command:

``` console
$ probe-rs-cli info
```

If you get an error, run the above program above a second time. Let us know what the output of the command(s) is.
