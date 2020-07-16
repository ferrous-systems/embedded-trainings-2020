## Interface

We have covered configurations and endpoints but what is an *interface*?

<p align="center">
  <img src="/usb-interface.svg" alt="TODO">
<p>

An interface is closest to a USB device's function. For example, a USB mouse may expose a single HID (Human Interface Device) interface to report user input to the host. USB devices can expose multiple interfaces within a configuration. For example, the nRF52840 Dongle could expose both a CDC ACM interface (AKA virtual serial port) *and* a HID interface; the first interface could be used for (`log::info!`-style) logs; and the second one could provide a RPC (Remote Procedure Call) interface to the host for controlling the nRF52840's radio.

An interface is made up of one or more *endpoints*. To give an example, a HID interface can use two (interrupt) endpoints, one IN and one OUT, for bidirectional communication with the host. A single endpoint cannot be used by more than one interface with the exception of the special "endpoint 0", which can be (and usually is) shared by all interfaces.

For detailed information about interfaces check section 9.6.5, Interface, of the USB specification.
