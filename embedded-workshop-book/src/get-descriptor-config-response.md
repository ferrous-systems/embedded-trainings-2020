# Response

So how should we respond to the host? As our only goal is to be enumerated we'll respond with the minimum amount of information possible.

✅ **First, check the request:**
Configuration descriptors are requested by *index*, not by their configuration value. Since we reported a single configuration in our device descriptor the index in the request must be zero. Any other value should be rejected by stalling the endpoint (see section [Dealing with unknown requests: Stalling the endpoint](./unknown-requests.md#dealing-with-unknown-requests-stalling-the-endpoint) for more information).

✅ **Next, create and send a response:**
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
