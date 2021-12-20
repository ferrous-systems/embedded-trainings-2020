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