## Interface

We have covered configurations and endpoints but what is an *interface*?

<p align="center">
  <img src="/usb-interface.svg" alt="USB hierarchy diagram showing the relationship between configurations, interfaces and endpoints. The diagram consists of nested rectangles. In this version of the diagram the 'interface 0' rectangle and the 'bNumEndpoints' label are highlighted in blue.
The outermost rectangle is labeled 'device' and represents the complete USB device.
Inside the 'device' rectangle there is one rectangle labeled 'configuration 1'; this rectangle has a 'parallel lines' symbol that indicates there may be more than one configuration instance; the symbol is labeled 'bNumConfigurations=1' indicating that this device has only one configuration.
Inside the 'configuration 1' rectangle there are two rectangles labeled 'control endpoint' and 'interface 0'. Inside the 'control endpoint' rectangle there are two rectangles labeled 'endpoint 0 IN' and 'endpoint 0 OUT. The 'interface 0' rectangle has a 'parallel lines' symbol that indicates there may be more than one interface instance; the symbol is labeled 'bNumInterfaces=1' indicating that this configuration has only one interface.
Inside the 'interface 0' rectangle there are three rectangles labeled 'endpoint 1 IN', 'endpoint 2 IN' and 'endpoint 2 OUT'. Between these three rectangle there is a label that says 'bNumEndpoints=3'; it indicates that this interface has only three endpoints.">
<p>

An interface is closest to a USB device's function. For example, a USB mouse may expose a single HID (Human Interface Device) interface to report user input to the host. USB devices can expose multiple interfaces within a configuration. For example, the nRF52840 Dongle could expose both a CDC ACM interface (AKA virtual serial port) *and* a HID interface; the first interface could be used for (`defmt::println!`-style) logs; and the second one could provide a RPC (Remote Procedure Call) interface to the host for controlling the nRF52840's radio.

An interface is made up of one or more *endpoints*. To give an example, a HID interface can use two (interrupt) endpoints, one IN and one OUT, for bidirectional communication with the host. A single endpoint cannot be used by more than one interface with the exception of the special "endpoint 0", which can be (and usually is) shared by all interfaces.

For detailed information about interfaces check section 9.6.5, Interface, of the USB specification.
