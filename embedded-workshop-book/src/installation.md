# Installation Instructions

## VS Code

**Windows**: Go to [https://code.visualstudio.com](https://code.visualstudio.com) and run the installer.

**Linux**: Check your Linux distribution package manager (example below). If it's not there, follow the instructions on [https://code.visualstudio.com/docs/setup/linux](https://code.visualstudio.com/docs/setup/linux).

``` console
$ # Arch Linux
$ sudo pacman -S code
```

**macOS**: Go to [https://code.visualstudio.com](https://code.visualstudio.com) and click on "Download for Mac"

## OS specific dependencies

### Linux only: USB

Some of our tools depend on `pkg-config` and `libudev.pc`. Ensure you have the proper packages installed; on Debian based distributions you can use:

``` console
$ sudo apt-get install libudev-dev libusb-1.0-0-dev
```

To access the USB devices as a non-root user, follow these steps:

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
$ cat /etc/udev/rules.d/50-oxidize-global.rules
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

### Windows only: Zadig JLink driver

On Windows you'll need to associate the nRF52840 Development Kit's USB device to the WinUSB driver.

To do that connect the nRF52840 DK to your PC using micro-USB port J2 (as done before) then download and run the [Zadig] tool.

[Zadig]: https://zadig.akeo.ie/

In Zadig's graphical user interface,

1. Select the 'List all devices' option from the Options menu at the top.

2. From the device (top) drop down menu select "BULK interface (Interface 2)"

3. Once that device is selected, `1366 1015` should be displayed in the USB ID field. That's the Vendor ID - Product ID pair.

4. Select 'WinUSB' as the target driver (right side)

5. Click "Install WinUSB driver". The process may take a few minutes to complete.

> You do not need to do anything for the **nRF52840 Dongle** device.

## Rust and tooling

### Base Rust installation

Go to [https://rustup.rs](https://rustup.rs/) and follow the instructions.

**Windows**: *Do* install the optional components of the [C++ build tools package](https://visualstudio.microsoft.com/visual-cpp-build-tools/). The installation size may take up to 2 GB of disk space.

### Rust Analyzer

**All**: Open VS Code and look for [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) in the marketplace (bottom icon in the left panel). Then install it.

**Windows**: It's OK to ignore the message about `git` not being installed, if you get one!

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

### Cargo subcommands

Install version v0.8.0 of the [`cargo-flash`](https://crates.io/crates/cargo-flash) and [`cargo-embed`](https://crates.io/crates/cargo-embed) subcommands, as well as the [`cargo-binutils`](https://crates.io/crates/cargo-binutils) set of subcommands and the [`cargo-bloat`](https://crates.io/crates/cargo-bloat) subcommand using the following Cargo commands:

``` console
$ cargo install cargo-flash --version 0.8.0 -f
(..)
Installed package `cargo-flash v0.8.0` (..)

$ cargo install cargo-embed --version 0.8.0 -f
(..)
Installed package `cargo-embed v0.8.0` (..)

$ cargo install cargo-binutils
(..)
Installed package `cargo-binutils v0.3.0` (..)

$ cargo install cargo-bloat
(..)
Installed package `cargo-bloat v0.9.3` (..)

$ cargo install probe-run
(..)
Installed package `probe-run v0.1.3` (..)
```

## Python

**Windows**: Go to [https://www.python.org/downloads/](https://www.python.org/downloads/) and run the Python *3* installer

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

## nrf tools

### nrfutil

**All**: Open a terminal and install [nrfutil](https://github.com/NordicSemiconductor/pc-nrfutil) as follows. *If you are familiar with python, it is advised to do this in a [virtual environment](https://docs.python.org/3/library/venv.html).*

``` console
$ pip install nrfutil
(..)

$ nrfutil version
nrfutil version 6.1.0
```

### `nrf-recover`

Some nRF52840 devices, specially older revisions, may have parts of their Flash memory locked. To unlock the memory use the [`nrf-recover`](https://crates.io/crates/nrf-recover) tool.

This is only relevant to the nRF52840 Development Kit. First connect the nRF52840 DK to your PC using micro-USB J2 (as done before) then run the following commands:

``` console
$ cargo install nrf-recover

$ nrf-recover -y
Starting mass erase...
Mass erase completed, chip unlocked
```