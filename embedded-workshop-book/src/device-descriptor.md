# Device Descriptor

<p align="center">
  <img src="/usb-device.svg" alt="TODO">
<p>

After receiving a GET_DESCRIPTOR request during the SETUP stage the device needs to respond with a *descriptor* during the DATA stage.

A descriptor is a binary encoded data structure sent by the device to the host. The device descriptor, in particular, contains information about the device, like its product and vendor identifiers and how many *configurations* it has. The format of the device descriptor is specified in section 9.6.1, Device, of the USB specification.

As far as the enumeration process goes, the most relevant fields of the device descriptor are the number of configurations and `bcdUSB`, the version of the USB specification the devices adheres to. In `bcdUSB` you should report compatibility with USB 2.0.

What about (the number of) configurations?

A *configuration* is akin to an operation mode. USB devices usually have a single configuration that will be the only mode in which they'll operate, for example a USB mouse will always act as a USB mouse. Some devices, though, may provide a second configuration for the purpose of firmware upgrades. For example a printer may enter DFU (Device Firmware Upgrade) mode, a second *configuration*, so that a user can update its firmware; while in DFU mode the printer will not provide printing functionality.

The specification mandates that a device must have at least one available configuration so we can report a single configuration in the device descriptor.
