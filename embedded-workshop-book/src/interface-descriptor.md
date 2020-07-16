## Interface descriptor

<p align="center">
  <img src="/usb-interface.svg" alt="TODO">
<p>

The interface descriptor describes one of the device interfaces to the host. The descriptor contains the following information about a particular interface:

- its interface number -- this is a zero-based index
- its alternate setting -- this allows configuring the interface
- its number of endpoints
- class, subclass and protocol -- these define the interface (HID, or TTY ACM, or DFU, etc.) according to the USB specification

The number of endpoints can be zero and endpoint zero must not be accounted when counting endpoints.
> The full format of the interface descriptor is specified in section 9.6.5, Interface, of the USB specification.
