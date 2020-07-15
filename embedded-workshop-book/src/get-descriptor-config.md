# Handling GET_DESCRIPTOR Configuration Requests

When the host issues a GET_DESCRIPTOR *Configuration* request the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

 A GET_DESCRIPTOR Configuration request is a GET_DESCRIPTOR request where the descriptor type encoded in the high bit of `wValue` is CONFIGURATION.

We have covered configurations and endpoints but what is an *interface*?

## Interface

An interface is closest to a USB device's function. For example, a USB mouse may expose a single HID (Human Interface Device) interface to report user input to the host. USB devices can expose multiple interfaces within a configuration. For example, the nRF52840 Dongle could expose both a CDC ACM interface (AKA virtual serial port) *and* a HID interface; the first interface could be used for (`log::info!`-style) logs; and the second one could provide a RPC (Remote Procedure Call) interface to the host for controlling the nRF52840's radio.

An interface is made up of one or more *endpoints*. To give an example, a HID interface can use two (interrupt) endpoints, one IN and one OUT, for bidirectional communication with the host. A single endpoint cannot be used by more than one interface with the exception of the special "endpoint 0", which can be (and usually is) shared by all interfaces.

For detailed information about interfaces check section 9.6.5, Interface, of the USB specification.

## Configuration descriptor

The configuration descriptor describes one of the device configurations to the host. The descriptor contains the following information about a particular configuration:

- the total length of the configuration: this is the number of bytes required to transfer this configuration descriptor and the interface and endpoint descriptors associated to it
- its number of interfaces -- must be >= 1
- its configuration value -- this is *not* an index and can be any non-zero value
- whether the configuration is self-powered
- whether the configuration supports remote wakeup
- its maximum power consumption

> The full format of the configuration descriptor is specified in section 9.6.3, Configuration, of the USB specification.

## Interface descriptor

The interface descriptor describes one of the device interfaces to the host. The descriptor contains the following information about a particular interface:

- its interface number -- this is a zero-based index
- its alternate setting -- this allows configuring the interface
- its number of endpoints
- class, subclass and protocol -- these define the interface (HID, or TTY ACM, or DFU, etc.) according to the USB specification

The number of endpoints can be zero and endpoint zero must not be accounted when counting endpoints.
> The full format of the interface descriptor is specified in section 9.6.5, Interface, of the USB specification.

## Endpoint descriptor

We will not need to deal with endpoint descriptors in this workshop but they are specified in section 9.6.6, Endpoint, of the USB specification.

## Response

So how should we respond to the host? As our only goal is to be enumerated we'll respond with the minimum amount of information possible.

**First, check the request:**  
Configuration descriptors are requested by *index*, not by their configuration value. Since we reported a single configuration in our device descriptor the index in the request must be zero. Any other value should be rejected by stalling the endpoint (see section [Dealing with unknown requests: Stalling the endpoint](./unknown-requests.md#dealing-with-unknown-requests-stalling-the-endpoint) for more information).

**Next, create and send a response:**  
The response should consist of the configuration descriptor, followed by interface descriptors and then by (optional) endpoint descriptors. We'll include a minimal single interface descriptor in the response. Since endpoints are optional we will include none.


The configuration descriptor and one interface descriptor will be concatenated in a single packet so this response should be completed in a single DATA stage.

The configuration descriptor in the response should contain these fields:

- `bLength = 9`, the size of this descriptor (see table 9-10 in the USB spec)
- `bDescriptorType = 2`, configuration descriptor (see table 9-5 in the USB spec)
- `wTotalLength = 18` = one configuration descriptor (9 bytes) and one interface descriptor (9 bytes)
- `bNumInterfaces = 1`, a single interface (the minimum value)
- `bConfigurationValue = 42`, any non-zero value will do
- `iConfiguration = 0`, string descriptors are not supported
- `bmAttributes { self_powered: true, remote_wakeup: false }`, self-powered due to the debugger connection
- `bMaxPower = 250` (500 mA), this is the maximum allowed value but any (non-zero?) value should do

The interface descriptor in the response should contain these fields:

- `bLength = 9`, the size of this descriptor (see table 9-11 in the USB spec)
- `bDescriptorType = 4`, interface descriptor (see table 9-5 in the USB spec)
- `bInterfaceNumber = 0`, this is the first, and only, interface
- `bAlternateSetting = 0`, alternate settings are not supported
- `bNumEndpoints = 0`, no endpoint associated to this interface (other than the control endpoint)
- `bInterfaceClass = bInterfaceSubClass = bInterfaceProtocol = 0`, does not adhere to any specified USB interface
- `iInterface = 0`, string descriptors are not supported

Again, we strongly recommend that you use the `usb2::configuration::Descriptor` and `usb2::interface::Descriptor` abstractions here. Each descriptor instance can be transformed into its byte representation using the `bytes` method -- the method returns an array. To concatenate both arrays you can use an stack-allocated [`heapless::Vec`] buffer. If you haven't the `heapless` crate before you can find example usage in the the `src/bin/vec.rs` file.

[`heapless::Vec`]: https://docs.rs/heapless/0.5.5/heapless/struct.Vec.html
