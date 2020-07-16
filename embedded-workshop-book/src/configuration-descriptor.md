# Configuration descriptor

<p align="center">
  <img src="/usb-configuration.svg" alt="TODO">
<p>

The configuration descriptor describes one of the device configurations to the host. The descriptor contains the following information about a particular configuration:

- the total length of the configuration: this is the number of bytes required to transfer this configuration descriptor and the interface and endpoint descriptors associated to it
- its number of interfaces -- must be >= 1
- its configuration value -- this is *not* an index and can be any non-zero value
- whether the configuration is self-powered
- whether the configuration supports remote wakeup
- its maximum power consumption

> The full format of the configuration descriptor is specified in section 9.6.3, Configuration, of the USB specification.
