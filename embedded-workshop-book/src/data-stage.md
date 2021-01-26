# USB-3: DATA Stage

The next step is to respond to the GET_DESCRIPTOR request with a device descriptor.

✅ Open the file `src/bin/usb-3.rs`. Implement the response to the GET_DESCRIPTOR request. Use the following guide for assistance.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

 To do this we'll use the `dk::usb::Ep0In` abstraction -- we'll look into what the abstraction does in a future section; for now we'll just use it.

An instance of this abstraction is available in the `board` value (`#[init]` function). The first step is to make this `Ep0In` instance available to the `on_event` function.

The `Ep0In` API has two methods: `start` and `end` (also see their API documentation). `start` is used to start a DATA stage; this method takes a *slice of bytes* (`[u8]`) as argument; this argument is the response data. The `end` method needs to be called after `start`, when the EP0DATADONE event is raised, to complete the control transfer. `Ep0In` will automatically issue the STATUS stage that must follow the DATA stage.

To implement responding to a GET_DESCRIPTOR Device request, extend `usb-3.rs` so that it uses `Ep0In` to respond to the `GET_DESCRIPTOR Device` request (and only to that request). The response must be a device descriptor with its fields set to these values:

- `bLength = 18`, the size of the descriptor (must always be this value)
- `bDescriptorType = 1`, device descriptor type (must always be this value)
- `bDeviceClass = bDeviceSubClass = bDeviceProtocol = 0`, these are unimportant for enumeration
- `bMaxPacketSize0 = 64`, this is the most performant option (minimizes exchanges between the device and the host) and it's assumed by the `Ep0In` abstraction
- `idVendor = consts::VID`, value expected by `cargo xtask usb-list` (\*)
- `idProduct = consts::PID`, value expected by `cargo xtask usb-list` (\*)
- `bcdDevice = 0x0100`, this means version 1.0 but any value should do
- `iManufacturer = iProduct = iSerialNumber = None`, string descriptors not supported
- `bNumConfigurations = 1`, must be at least `1` so this is the minimum value

>(\*) the `common` crate refers to the crate in the `advanced/common` folder. It is already part of the `firmware` crate dependencies.

Although you can create the device descriptor by hand as an array filled with magic values we *strongly* recommend you use the `usb2::device::Descriptor` abstraction. The crate is already in the dependency list of the project; you can open its API documentation with the following command: `cargo doc -p usb2 --open`.

> NOTE: the `usb2::device::Descriptor` struct does not have `bLength` and `bDescriptorType` fields. Those fields have fixed values according to the USB spec so you cannot modify or set them. When `bytes()` is called on the `Descriptor` value the returned array, the binary representation of the descriptor, will contain those fields set to their correct value.

Note that the device descriptor is 18 bytes long but the host may ask for fewer bytes (see `wlength` field in the SETUP data). In that case you must respond with the amount of bytes the host asked for. The opposite may also happen: `wlength` may be larger than the size of the device descriptor; in this case your answer must be 18 bytes long (do *not* pad the response with zeroes).

Don't forget to also handle the `EP0DATADONE` event!

Once you have successfully responded to the GET_DESCRIPTOR Device request you should get logs like these (if you are logging like `usb-3` does):

``` console
INFO:usb_3 -- USB: UsbReset @ 342.071532ms
INFO:usb_3 -- USB: UsbEp0Setup @ 414.855956ms
INFO:usb_3 -- SETUP: bmrequesttype: 128, brequest: 6, wlength: 64, windex: 0, wvalue: 256
INFO:usb_3 -- GET_DESCRIPTOR Device [length=64]
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_3 -- USB: UsbEp0DataDone @ 415.222166ms
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_3 -- USB: UsbReset @ 465.637206ms
INFO:usb_3 -- USB: UsbEp0Setup @ 538.208007ms
INFO:usb_3 -- SETUP: bmrequesttype: 0, brequest: 5, wlength: 0, windex: 0, wvalue: 27
ERROR:usb_3 -- unknown request (goal achieved if GET_DESCRIPTOR Device was handled)
INFO:dk -- `dk::exit() called; exiting ...`
```

A solution to this exercise can be found in `src/bin/usb-3-solution.rs`.
