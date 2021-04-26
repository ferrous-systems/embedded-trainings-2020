# Installation Instructions

## Workshop Materials

Clone and change into the [workshop git repository](https://github.com/ferrous-systems/embedded-trainings-2020):

```console
$ git clone https://github.com/ferrous-systems/embedded-trainings-2020.git
$ cd embedded-trainings-2020
```

The workshop repository contains all workshop materials, i.e. code snippets, custom tools and the source for this handbook.

All programming will take place in its `beginner/` and `advanced/` subfolders.

## VS Code

**Windows**: Go to [https://code.visualstudio.com](https://code.visualstudio.com) and run the installer.

**Linux**: Follow the instructions for your distribution on [https://code.visualstudio.com/docs/setup/linux](https://code.visualstudio.com/docs/setup/linux).

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

### Better TOML

**All**: For better handling of `Cargo.toml` files, we recommend you install [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) if you're using VS Code.

### Rust Cross compilation support

**All**: Run this command in a terminal:

``` console
$ rustup +stable target add thumbv7em-none-eabihf
```

### ELF analysis tools

**All**: Run these commands in a terminal:

``` console
$ cargo install cargo-binutils

$ rustup +stable component add llvm-tools-preview
```

### General purpose tools

Install the [`flip-link`](https://crates.io/crates/flip-link) and [`probe-run`](https://crates.io/crates/probe-run) tools using the following Cargo commands:

``` console

$ cargo install probe-run
(..)
Installed package `probe-run v0.1.8` (..)

$ cargo install flip-link
(..)
Installed package `flip-link v0.1.2` (..)
```

## Python

**Windows**: Go to [https://www.python.org/downloads/](https://www.python.org/downloads/) and run the Python *3* installer

- in the installer check the "add Python 3.x to PATH" box
- also run the "Disable path length limit" action at the end, if you are on Windows 10 and the option is displayed to you

**Linux**: Install `pip` using the package manager; this will also install Python.

``` console
$ # If you're using Arch Linux
$ sudo pacman -S python-pip

$ # If you're using Ubuntu
$ sudo apt-get install python3-pip
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

**NOTE** as of version 6.1.0 nrfutil indicates that it does "not" support Python 3.9 in its pip manifest. Due to this, the above `pip install` command will fail if you have a Python 3.9 interpreter installed. Here's how to work around the issue:

*start of nrfutil python 3.9 workaround*

``` console
$ # these steps can also be done via a graphical interface

$ # fetch the source code of version 6.1.0
$ # or instead of curl you can enter the URL into your web browser
$ curl -LO https://github.com/NordicSemiconductor/pc-nrfutil/archive/v6.1.zip

$ # unzip the code
$ unzip v6.1.zip

$ # go into the new folder
$ cd pc-nrfutil-6.1
```

Apply the following 2 patches (can also be done manually by editing these 2 files: `requirements.txt` and `setup.py`)

``` diff
--- a/requirements.txt  2021-01-05 10:50:12.611556607 +0100
+++ b/requirements.txt  2021-01-05 10:50:09.718226891 +0100
@@ -4,7 +4,7 @@
 ecdsa
 intelhex
 libusb1
-pc_ble_driver_py >= 0.14.2
+pc_ble_driver_py
 piccata
 protobuf
 pyserial
```

``` diff
--- a/setup.py  2021-01-05 10:49:56.014910707 +0100
+++ b/setup.py  2021-01-05 10:50:26.004873175 +0100
@@ -148,7 +148,7 @@
                      '../libusb/x86/libusb-1.0.dll', '../libusb/x64/libusb-1.0.dll',
                      '../libusb/x64/libusb-1.0.dylib', '../libusb/LICENSE']
     },
-    python_requires='>=3.6, <3.9',
+    python_requires='>=3.6, <3.10',
     install_requires=reqs,
     zipfile=None,
     tests_require=[
```

``` console
$ patch -p1 < requirements.patch
$ patch -p1 < setup.patch
```

Then install the patched `nrfutil`

``` console
$ pip install .

$ # verify installation
$ nrfutil version
nrfutil version 6.1.0
```

*end of nrfutil python 3.9 workaround*
