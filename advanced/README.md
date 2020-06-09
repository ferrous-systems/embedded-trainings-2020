# `advanced`

> Advanced workshop

In this workshop we'll build a toy USB device application that gets enumerated by the host.

The embedded application will run in a fully event driven fashion: only doing work when the host asks it to.

## Code organization

The `advanced` folder contains both "host" code, code that will run on the host, and "firmware" code, code that will run on the nRF52840 SoC. "host" and "firmware" source code has been placed in different Cargo workspaces so that each can be compiled with different compilation profiles. The `host` workspace will be natively compiled, whereas the `firmware` workspace will be cross-compiled for the ARM Cortex-M architecture.

``` console
$ cd advanced

$ tree -L 1 .
.
├── common
├── firmware
├── host
└── README.md
```

In addition to these two workspaces there's a third folder called "common". This folder contains `no_std` code that be depended on by either "host" code or "firmware" code.

## Listing USB devices

Open the `advanced/host/lsusb` folder in VS Code and run the program.

``` console
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
```

The goal is to get the nRF52840 SoC to show in this list. The embedded application will use the vendor ID (VID) and product ID (PID) defined in `advanced/common/consts`; the `lsusb` tool will also look for a USB device with that VID/PID and highlight it in the output.

``` console
$ # expected output
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 059: ID 0000:0000 <- !
```

## Hello, world!

0. Open the `tools/dk-run` folder and run `cargo install --path . -f` to install the `dk-run` tool.

1. Open the `advanced/firmware` folder in VS Code.

``` console
$ # or use "File > Open Folder" in VS Code
$ code advanced/firmware
```

2. Open the `src/bin/hello.rs` file.

3. Click the "Run" button

The `firmware` workspace has been configured to cross-compile applications to the ARM Cortex-M architecture and then run them through the `dk-run` custom Cargo runner. The `dk-run` tool will load and run the embedded application on the microcontroller and collect logs from the microcontroller.

The `dk-run` process will terminate when the microcontrollers enters the "halted" state. From the embedded application, one can enter the "halted" state using the `asm::bkpt` function.

## RTIC hello

RTIC, Real Time on Integrated Circuits, is a framework for building evented, time sensitive applications.

1. Open the `advanced/apps` folder in VS Code.

``` console
$ # or use "File > Open Folder" in VS Code
$ code beginner/apps
```

2. Open the `src/bin/rtic-hello.rs` file.

3. Click the "Run" button

> TODO explain differences between `hello` and `rtic-hello`

## Dealing with registers

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-events.rs` file.

> TODO explain the basics of the svd2rust API

> TODO explain what the code in `init` is doing

## Event handling

"Run" the `rtic-events` application.

Connect a micro-USB cable to your PC/laptop then connect the other end to the DK (TODO specify port).

> TODO explain the event handler

## Adding state

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-resources.rs` file.

> TODO

You should always disconnect the device from the host before halting the device. Otherwise, the host will observe an unresponsive USB device and try power cycling the whole USB hub / bus.

## USB primer

Some basics about the USB protocol. The protocol is complex so we'll leave out many details and focus on the concepts required to get enumeration working.

A USB device, the nRF52840 in our case, can be one of these three states: the Default state, the Address state or the Configured state. 

After being powered the device will start in the Default state. The enumeration process will take the device from the Default state to the Address state. As a result of the enumeration process the device will be assigned an address, in the range `1..=127`, by the host.

Each OS may perform the enumeration process slightly differently but the process will always involve these host actions:

- USB reset. This will put the device in the Default state, regardless of what state it was in.
- GET_DESCRIPTOR request to get the device descriptor.
- SET_ADDRESS request to assign an address to the device.

The device descriptor is a binary encoded data structure sent by the device to the host that contains information about the device, like its product and vendor identifiers and how many *configurations* it has.

A *configuration* is basically an operation mode or profile. The USB device may act as a HID device in one configuration but as a CDC ACM device (serial terminal emulation over USB) in another configuration. The number of configurations the device supports is part of the device descriptor. 

> TODO add/info: drivers are bound to interfaces, not devices

> FIXME configuration descriptor requires at least one interface

Like the device descriptor, the configuration descriptor is also a binary encoded data structure sent by the device to the host. This descriptor contains information about the *interfaces* and *endpoints* the configuration uses. An *endpoint* is similar to a UDP or TCP port on a single PC: it allows logical multiplexing on a single physical USB bus. An *interface* is a logical grouping of endpoints.

In this workshop we'll only deal with the control endpoint 0, which is mandatory on all USB devices. Endpoints have directions: in an OUT endpoint data flows from the host to the device; in an IN endpoint data flows from the device to the host. The control endpoint 0 actually refers to two endpoints: endpoint 0 IN (EP0IN) and endpoint 0 OUT (EP0OUT). Although both should be implemented, it's usually sufficient to implement EP0IN to get enumeration working.

A USB device must report at least one configuration. The control endpoint, EP0IN and EP0OUT, however does not need to described in the configuration descriptor so we can report 0 endpoints and 0 interfaces in the configuration descriptor. 

## Dealing with USB events

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-usb-1.rs` file.

The USB peripheral contains a series of registers, called EVENTS registers, that indicate the reason for entering the USBD event handler. These events must be handled by the application to complete the enumeration process.

In this stage you will need to deal with the following USB events until you reach the EP0STATUS event.

- `USBRESET`. This event indicates that the host issued a USB reset signal. According to the USB specification this will move the device from any state to the `Default` state. Where are not dealing with any other state so doing nothing in response to this event is OK for now.

- `EP0SETUP`. The USBD peripheral has detected the SETUP stage of a control transfer. If you get to this point move to the next section.

- `EP0SETUP`. The USBD peripheral is signaling the end of the DATA stage of a control transfer. You won't encounter this event just yet. 

## SETUP stage

Control transfers over endpoint 0 consists of three stages: a SETUP stage, an optional DATA stage and a STATUS stage.

The SETUP stage consists of the host sending a 8 byte header to the device. This header describes the host request: is the host expecting a response (data) from the device, is the host about to send some data to the device or is this a request with no data payload? The SETUP stage conveys this information.

The nRF52840 USBD peripheral will parse this header and store its contents in the following registers: BMREQUESTTYPE, BREQUEST, WVALUE{L,H}, WINDEX{L,H} and WLENGTH{L,H}. These registers match the logical division of the setup packet, in *fields*, as specified in the USB 2.0 specification. The fields that start with the letter 'b' are 1-byte large; the ones that start with the letter 'w' are 2-bytes large.

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-usb-2.rs` file. 

The task here is to write a SETUP packet parser in the `common/usb` crate and reach the GOAL comment when executing the program. The starter code has already read the relevant registers using helper functions.

To implement the parser, refer to table 9-3 under section 9.4 of the USB 2.0 specification (page 250). Note that at this stage you only need to be able to parse the `GetDescriptor` variant of the `Request` enum and within that variant you only need to handle the `Device` variant of the `Descriptor` enum.

You should develop this part completely on the host. Switch the workspace to the `common/usb` directory and run the unit tests on the host using Rust Analyzer's "Test" button.

## Device descriptor

After receiving a GET_DESCRIPTOR request during the SETUP stage the device needs to respond with a descriptor during the DATA stage.

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-usb-3.rs` file. 

This starter code parses the GET_DESCRIPTOR request and sends back a zero-byte response to the host using the `Ep0In` abstraction, which we'll cover in the next section. The starter code is wrong because a zero-byte response is not a valid descriptor.

The task in this section is to build a device descriptor using the `usb2::device::Descriptor` API, turn that `Descriptor` instance into a sequence of bytes and respond with that. Note that the device must send back at most `length` bytes to the host; if the byte representation of `Descriptor` is longer than `length` then the slice must be truncated to `length` bytes.

As for the contents of the descriptor you should use these values:

- Vendor ID: `consts::VID`
- Product ID: `consts::PID`
- Max packet size for endpoint 0: 64 bytes
- Number of configurations: 1
- bcdDevice: `0x01_00`, which means version "1.00"
- everything else can be set to `0` or `None` 

We suggest you check the API docs of the `usb2` crate by running the command `cargo doc -p usb2 --open`. This should open the API docs in your browser.

More information about the fields of the device descriptor can be found in section 9.6.1 of the USB 2.0 spec.

After that has been fixed you'll reach the EP0DATADONE event. It indicates that the data transfer has completed. You must call the `Ep0In.end` method at that point. After that you'll likely get a new request from the host.

## DMA

Let's zoom into the `Ep0In` abstraction next. You can use the "go to definition" to see the implementation of the `Ep0In.start` method. What this method does is start a DMA transfer to send `bytes` to the host. The data (`bytes`) is first copied into an internal buffer and then the DMA is configured to move the data from that internal buffer to the USBD peripheral.

The signature of the `start` method does *not* ensure that (a) `bytes` won't be deallocated before the DMA transfer is over (e.g. `bytes` could be pointing into the stack) or that (b) `bytes` won't be modified right after the DMA transfer starts (this would be a data race in the general case). For these two safety reasons the API is implemented using an internal buffer. The internal buffer has a `'static` lifetime so it's guaranteed to never be deallocated -- this prevent issue (a). The `busy` flag prevents any further modification to the buffer -- from the public API -- while the DMA transfer is in progress.

Apart from thinking about lifetimes and explicit data races in the surface API one must internally use memory fences to prevent reordering of memory operations (e.g. by the compiler), which can also cause data races. DMA transfers run in parallel to the instructions performed by the processor and are "invisible" to the compiler.

In the implementation of the `start` method, data is copied from `bytes` to the internal buffer, `memcpy` operation, and then the DMA transfer is started. The compiler sees the start of the DMA transfer as an unrelated memory operation so it may move the `memcpy` to *after* the DMA transfer has started. This reordering results in a data race: the processor modifies the internal buffer while the DMA is reading data out from it. To avoid this reordering a memory fence, `dma_start`, is used. The fence pairs with the store operation that starts the DMA transfer and prevents the previous `memcpy`, and any other memory operation, from being move to *after* the store operation.

Another memory fence, `dma_end`, is need at the end of the DMA transfer. In the general case, this prevents instruction reordering that would result in the processor accessing the internal buffer *before* the DMA transfer has finished. This is particularly problematic with DMA transfers that modify a region of memory which the processor intends to read after the transfer.

Not relevant to the DMA operation but relevant to the USB specification, the `start` method sets a shortcut in the USBD peripheral to issue a STATUS stage right after the DATA stage is finished. Thanks to this it is not necessary to manually start a STATUS stage after calling the `end` method.

## Error handling: stalling the endpoint

The USB specification defines a device-side procedure for "stalling a endpoint", which amounts to the device telling the host that it doesn't support some request. This procedure should be used to deal with invalid requests, requests whose SETUP stage doesn't match any USB 2.0 standard request, and requests not supported by the device, for instance the SET_DESCRIPTOR request is not mandatory.

Open the `advanced/firmware` folder in VS Code; then open the `src/bin/rtic-usb-4.rs` file. 

In this starter code the code that handles the SETUP stage of endpoint 0 has been refactored into a separate function, `ep0setup`, that uses Rust's built-in error handling features. This function will return the `Err` variant when it encounters an invalid request or a request that is not supported.

You can use the `usbd:ep0stall` helper function or write 1 to the TASKS_EP0STALL register to stall endpoint 0.

## State

The starter code in `src/bin/rtic-usb-4.rs` also passes a `State` variable to the `ep0setup` function. This variable tracks the state of the USB device, which can be one of `Default`, `Address` or `Configured`.

From this point you'll need to track the state of the device in software to be able to correctly respond to the host requests. 

The handling of the USB reset condition (`Event::UsbReset`) will need to be updated. According to the USB specification this needs to change the device state, from any state, to the `Default` state.

## SET_ADDRESS

This request changes the device's state as follows:

- If the device is in the `Default` state, then
  - if the requested address was `0` (`None` in the `usb2` API) then the device should stay in the `Default` state
  - otherwise the device should move to the `Address` state
  
- If the device is in the `Address` state, then
  - if the requested address was `0` (`None` in the `usb2` API) then the device should return to the `Default` state
  - otherwise the device should remain in the `Address` state but start using the new address

- If the device is in the `Configured` state this request results in "unspecified" behavior according to the USB specication. You should stall the endpoint in this case.
  
According to the USB specification the device needs to respond to this request with a STATUS stage -- the DATA stage is omitted. The nRF52840 USBD peripheral will automatically issue the STATUS stage and switch to listening only to the requested address (see the USBADDR register) so no further interaction with the USBD peripheral is required for this request.

For more details, read the introduction of section 6.35.9 of the nRF52840 Product Specification 1.0 (pages 486 and 487).

## Configuration descriptors

When the host issues a GET_DESCRIPTOR request to request a configuration descriptor the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

For simplicity we'll report that the device has zero interfaces and zero endpoints, other than the control endpoint 0 which doesn't need to be reported. So in response to a `GET_DESCRIPTOR Configuration 0` request the device should respond with the binary representation of a `usb2::configuration::Descriptor` instance. The instance should contain these values:

- wTotalLength: `usb2::configuration::Descriptor::SIZE`, as just the configuration descriptor is being reported back
- bNumInterfaces: `0`
- bConfigurationValue: `1`
- bmAttributes: `{ self_powered: true, remote_wakeup: false }`
- bMaxPower: `250`, equivalent to 500 mA

Note that the index of the `GET_DESCRIPTOR Configuration` request must be `0` because the device reported a single configuration in its device descriptor. Any other index is an "out of bounds" attempt and should be handled by stalling the endpoint.

## (Linux) SET_CONFIGURATION

On Linux, the host will sent a SET_CONFIGURATION request, after enumeration, to put the device in the `Configured` state. You'll need to handle the request according to the following logic:

- If the device is in the `Default` state, you should stall the endpoint.

- If the device is in the `Address` state, then
  - if the requested configuration value is 0 (`None` in the `usb2` API) then stay in the `Address` state
  - if the requested configuration value is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state
  - if the requested configuration value is not valid then stall the endpoint

- If the device is in the `Configured` state, then
  - if the requested configuration value is 0 (`None` in the `usb2` API) then return to the `Address` state
  - if the requested configuration value is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state with the new configuration value
  - if the requested configuration value is not valid then stall the endpoint
  
In all the cases where you did not stall the endpoint (returned `Err`) you'll need to acknowledge the request by starting a STATUS stage. This can be done by writing 1 to the TASKS_EP0STATUS register.

For more details, read the section 9.4.7 'SET_CONFIGURATION' of the USB 2.0 specification.

## (homework) String descriptors

> NOTE(japaric) I hardly think the workshop can fit any more material within the allocated time frame but this is one, of many things, people could continue working on

## References

- [nRF52840 Product Specification 1.0](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.0.pdf)
- [Universal Serial Bus Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)
