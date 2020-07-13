# USB-2: SETUP Stage

At the end of program `usb-1` we received a EP0SETUP event. This event signals the *end* of the SETUP stage of a control transfer.  The nRF52840 USBD peripheral will automatically receive the SETUP data and store it in the registers BMREQUESTTYPE, BREQUEST, WVALUE{L,H}, WINDEX{L,H} and WLENGTH{L,H}.  
In `usb-2.rs`, you will find a short description of each register above the variable into which it should be read.

> For in-depth register documentation, refer to sections 6.35.13.31 to 6.35.13.38 of the [nRF52840 Product Specification][nrf product spec].
[nrf product spec]: https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf

When you need to write some `no_std` code that does not involve device-specific I/O you should consider writing it as a separate crate. This way, you can test it on your development machine (e.g. `x86_64`) using the standard `cargo test` functionality.

So that's what we'll do here. In `advanced/common/usb/lib.rs` you'll find starter code for writing a `no_std` SETUP data parser. The starter code contains some unit tests; you can run them with `cargo test` (from within the `usb` folder) or you can use Rust Analyzer's "Test" button in VS code.

The definition of `Descriptor::Configuration` as well as the associated test has been "commented out" using an `#[cfg(TODO)]` attribute because it is not handled by the firmware yet. Delete the `#[cfg(TODO)]` so that the unit tests can access it. This pattern is used for enum members and test functions throughout this workshop, so keep it in mind should you see it again.

Your task now is to parse the data of this SETUP stage. We will start with the GET_DESCRIPTOR request, which is described in detail in section 9.4.3 of the USB specification. All the constants you will need are described in Tables 9-3, 9-4 and 9-5.

The fields of a GET_DESCRIPTOR request are as follows:
- `bmRequestType` is 0b10000000
- `bRequest` is GET_DESCRIPTOR
- the high byte of `wValue` contains the descriptor type, whereas the low byte contains the descriptor index
- `wIndex` is set to 0 for our purposes

You will also find this information in the `// TODO implement ...` comment in the `Request::parse()` function of `lib.rs` file.
 > NOTE: If you'd like to learn more, take a look at Section 9.4.3 Get Descriptor of the USB specification.

To complete the task, proceed like this:

1. **Parse GET_DESCRIPTOR requests:**  
Modify `Request::parse()` in `advanced/common/usb/src/lib.rs` to recognize a GET_DESCRIPTOR request so that the `get_descriptor_device` test passes. Note that the parser already handles SET_ADDRESS requests.

    - check table 9-4 in the USB specification for Request Codes
    - remember that you can define binary literals by prefixing them with `0b`
    - you can use bit shifts (`>>`) and casts (`as u8`) to get the high/low bytes of a `u16`

See `advanced/common/usb/src/get-descriptor-device.rs` for a solution.

2. **Read incoming request information and pass it to the parser:**  
modify `usb-2.rs` to read `USBD` registers and parse the SETUP data when an EPSETUP event is received.
    - for a mapping of register names to the `USBD` API, check the entry for `nrf52840_hal::target::usbd` in the documentation you've created using `cargo doc`
    - remember that we've learned how to read registers in `events.rs`
    - you will need to put together the higher and lower bits of `wlength`, `windex` and `wvalue` to get the whole field
    - > Note: If you're using a Mac, you need to catch `SetAddress` requests returned by the parser as these are sent before the first GetDescriptor request. You can handle them by doing nothing.

3. when you have successfully received a GET_DESCRIPTOR request for a Device descriptor you are done. You should see an output like this:

``` console
INFO:usb_2 -- USB: UsbReset @ 438.842772ms
INFO:usb_2 -- USB: UsbEp0Setup @ 514.984128ms
...
INFO:usb_2 -- SETUP: bmrequesttype: 128, brequest: 6, wlength: 64, windex: 0, wvalue: 256
INFO:usb_2 -- GET_DESCRIPTOR Device [length=64]
INFO:usb_2 -- Goal reached; move to the next section
```

`wlength` / `length` can vary depending on the OS, USB port (USB 2.0 vs USB 3.0) or the presence of a USB hub so you may see a different value.

You can find a solution to step 1. in `advanced/common/usb/get-descriptor-device.rs`.  
You can find a solution to step 2. in `advanced/firmware/src/bin/usb-2-solution.rs`.
