# `advanced`

> Advanced workshop

In this workshop we'll build a toy USB device application that gets enumerated by the host.

The embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

## The nRF52840

Some details about the nRF52840 microcontroller that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

## The nRF52840 Development Kit

The development board we'll use has two USB ports: J2 and J3 -- you can find a description of the board in the top-level README of this repository -- and an on-board J-Link programmer / debugger. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port.

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

In addition to these two workspaces there's a third folder called "common". This folder contains `no_std` code that can be depended on by either "host" code or "firmware" code.

## Listing USB devices

In the `tools` folder you'll find `usb-list`: a minimal cross-platform version of the `lsusb` tool. Run it (`cargo run` from `tools/usb-list`) to list all USB devices.

``` console
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
```

The goal is to get the nRF52840 SoC to show in this list. The embedded application will use the vendor ID (VID) and product ID (PID) defined in `advanced/common/consts`; the `usb-list` tool will highlight the USB device that matches that VID PID pair.

``` console
$ # expected output
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 059: ID 2020:0705 <- nRF52840 on the nRF52840 Development Kit
```

## Hello, world!

First, open the `tools/dk-run` folder and run `cargo install --path . -f` to install the `dk-run` tool.

Next open the `advanced/firmware` folder in VS Code. If have already opened the root of the repository (`embedded-trainings-2020`) then please also open the `advanced/firmware` folder: right click the left side panel, select "Add folder to workspace" and add the `advanced/apps` folder.

Now, on the left side panel, open the `src/bin/hello.rs` file from under the `advanced/apps` folder.

Give Rust Analyzer some time to analyze the file and its dependency graph. When it's done, a "Run" button will appear over the `main` function -- you may need to edit the file contents to make the "Run" button appear.

Click the "Run" button to run the application on the microcontroller.

If you are not using VS code run the `cargo run --bin hello` command from the `advanced/firmware` folder.

> NOTE if you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear

The `firmware` workspace has been configured to cross-compile applications to the ARM Cortex-M architecture and then run them through the `dk-run` custom Cargo runner. The `dk-run` tool will load and run the embedded application on the microcontroller and collect logs from the microcontroller.

The `dk-run` process will terminate when the microcontroller enters the "halted" state. From the embedded application, one can enter the "halted" state using the `asm::bkpt` function. For convenience, an `exit` function is provided in the `dk` Hardware Abstraction Layer (HAL). This function is divergent like `std::process::exit` (`fn() -> !`) and can be used to halt the device and terminate the `dk-run` process.

Note that when the `dk-run` tool sees the device enter the halted state it will proceed to reset-halt the device. This is particularly important when writing USB applications because simply leaving the device in a halted state will make it appear as an unresponsive USB device to the host; some OSes (e.g. Linux) will try to make an unresponsive device respond by power cycling the entire USB bus -- this will cause all other USB devices on the bus to be re-enumerated. Reset-halting the device will cause it to be electrically disconnected from the host USB bus and avoid the "power cycle the USB bus" scenario.

## RTIC hello

RTIC, Real Time on Integrated Circuits, is a framework for building evented, time sensitive applications.

Open the `src/bin/rtic-hello.rs` file.

RTIC applications are written in RTIC's Domain Specific Language (DSL). The DSL extends Rust syntax with custom attributes like `#[init]` and `#[idle]`.

RTIC makes a clearer distinction between the application's initialization phase, the `#[init]` function, and the application's main loop or main logic, the `#[idle]` function. The initialization phase runs with interrupts disabled and interrupts are re-enabled before the `idle` function is executed. 

`rtic::app` is a procedural macro that generates extra Rust code, in addition to the user's functions. The fully expanded version of the macro can be found in the file `target/rtic-expansion.rs`. This file will contain the expansion of the procedural macro for the last compiled RTIC application.

If you look at the `rtic-expansion.rs` file generated for the build of the `rtic-hello` example you can confirm that interrupts are disabled during the execution of the `init` function.

``` rust
fn main() -> ! {
    rtfm::export::interrupt::disable();
    let late = init(init::Context::new(/* .. */));
    rtfm::export::interrupt::enable();
    idle(idle::Context::new(/* .. */))
}
```

## Dealing with registers

Open the `src/bin/rtic-events.rs` file.

In this and the next section we'll look into the RTIC's event handling features. To explore these features we'll use the action of connecting a USB cable to the DK's port J2 as the event we'd like to handle.

The example application enables the signaling of this "USB power" event in the `init` function. This is done using the low level register API generated by the [`svd2rust`] tool. The register API was generated from a SVD (System View Description) file, a file that describes all the peripherals and registers, and their memory layout, on a Cortex-M microcontroller.

[`svd2rust`]: https://crates.io/crates/svd2rust

In the `svd2rust` API, peripherals are represented as structs. The fields of each peripheral struct are the registers associated to that peripheral. Each register field exposes methods to `read` and `write` to the register in a single memory operation. 

The `read` and `write` methods take closure arguments. These closures in turn grant access to a "constructor" value, usually named `r` or `w`, which provides methods to modify the bitfields of a register. At the same time the API of these "constructors" prevent you from modifying the reserved parts of the register: you cannot write arbitrary values into registers; you can only write valid values into registers.

In Cortex-M devices interrupt handling needs to be enabled on two sides: on the peripheral side and on the core side. The register operations done in `init` take care of the peripheral side. The core side of the operation involves writing to the registers of the Nested Vector Interrupt Controller (NVIC) peripheral. This second part doesn't need to be in RTIC application because the framework takes care of it.

## Event handling

Below the `idle` function you'll see a `#[task]` handler (function). This *task* is bound to the POWER_CLOCK interrupt signal and will be executed, function call style, every time the interrupt signal is raised by the hardware.

"Run" the `rtic-events` application. Then connect a micro-USB cable to your PC/laptop then connect the other end to the DK (port J2). You'll see the "POWER event occurred" message after the cable is connected.

Note that all tasks will be prioritized over the `idle` function so the execution of `idle` will be interrupted (paused) by the `on_power_event` task. When the `on_power_event` task finishes (returns) the execution of the `idle` will be resumed. This will become more obvious in the next section.

## Task state

Open the `src/bin/rtic-resources.rs` file.

> TODO

You should always disconnect the device from the host before halting the device. Otherwise, the host will observe an unresponsive USB device and try power cycling the whole USB hub / bus.

## USB basics

Some basics about the USB protocol. The protocol is complex so we'll leave out many details and focus on the concepts required to get enumeration working.

A USB device, the nRF52840 in our case, can be one of these three states: the Default state, the Address state or the Configured state. 

After being powered the device will start in the Default state. The enumeration process will take the device from the Default state to the Address state. As a result of the enumeration process the device will be assigned an address, in the range `1..=127`, by the host.

Each OS may perform the enumeration process slightly differently but the process will always involve these host actions:

- USB reset. This will put the device in the Default state, regardless of what state it was in.
- GET_DESCRIPTOR request to get the device descriptor.
- SET_ADDRESS request to assign an address to the device.

The device descriptor is a binary encoded data structure sent by the device to the host. It contains information about the device, like its product and vendor identifiers and how many *configurations* it has.

A *configuration* is akin to an operation mode. USB devices usually have a single configuration that will be the only mode in which they'll operate, for example a USB mouse will always act as a USB mouse. Some devices, though, may provide a second configuration for the purpose of firmware upgrades. For example a printer may enter DFU (Device Firmware Upgrade) mode, a second *configuration*, so that a user can update its firmware; while in DFU mode the printer will not provide printing functionality.

Like the device descriptor, the configuration descriptor is also a binary encoded data structure sent by the device to the host. This descriptor contains information about the *interfaces* and *endpoints* the configuration exposes.

An interface is closest to a USB device's function. For example, a USB mouse may expose a single HID (Human Interface Device) interface to report user input to the host. USB devices can expose multiple interfaces. For example, the nRF52840 Dongle could expose both a CDC ACM interface (AKA virtual serial port) *and* a HID interface; the first interface could be used for (`log::info!`-style) logs; and the second one could provide a RPC (Remote Procedure Call) interface to the host for controlling the nRF52840's radio.

An interface is made up of one or more *endpoints*. An *endpoint* is similar to a UDP or TCP port on a PC in that they allow logical multiplexing of data over a single physical USB bus. Endpoints have directions: a endpoint can either be an IN endpoint or an OUT endpoint. The direction is always from the perspective of the host so in an IN endpoint data travels from the device to the host and in an OUT endpoint data travels from the host to the device. To give an example, a HID interface can use two (interrupt) endpoints, one IN and one OUT, for bidirectional communication with the host. A single endpoint cannot be used by more than one interface (with the exception of the special "endpoint 0").

Endpoints are identified by their address, a zero-based index, and direction. There are three types of non-zero endpoints ("endpoint 0" is special): bulk endpoints, interrupt endpoints and isochronous endpoints. Each endpoint type has different reliability and latency data transfer properties but it's not important to discuss them for this workshop.

"Endpoint 0", also known as the *control pipe*, actually refers to two endpoints: endpoint 0 IN and endpoint 0 OUT so the control pipe supports data transfers in both directions. The control pipe is mandatory: it must always be present and must always be active.

In this workshop we'll implement the minimal amount of functionality to make enumeration work. To that end you need to consider the following requirements:

- a USB device must support at least one configuration
- each configuration must expose at least one interface
- the control pipe (endpoint 0) must be implemented
- endpoint 0 is implicitly associated to all interfaces
- the number of endpoints bound to an interface can be zero -- endpoint 0 is never included in the endpoint count of an interface

Although the control pipe should be bidirectional, in practice to complete the enumeration data only needs to be transferred from the device to the host (IN direction). 

## Dealing with USB events

Open the `src/bin/rtic-usb-1.rs` file.

The USB peripheral contains a series of registers, called EVENTS registers, that indicate the reason for entering the USBD event handler. These events must be handled by the application to complete the enumeration process.

In this stage you will need to deal with the following USB events until you reach the EP0STATUS event.

- `USBRESET`. This event indicates that the host issued a USB reset signal. According to the USB specification this will move the device from any state to the `Default` state. Where are not dealing with any other state so doing nothing in response to this event is OK for now.

- `EP0SETUP`. The USBD peripheral has detected the SETUP stage of a control transfer. If you get to this point move to the next section.

- `EP0SETUP`. The USBD peripheral is signaling the end of the DATA stage of a control transfer. You won't encounter this event just yet. 

## SETUP stage

Control transfers over endpoint 0 consists of three stages: a SETUP stage, an optional DATA stage and a STATUS stage.

The SETUP stage consists of the host sending a 8 byte header to the device. This header describes the host request: is the host expecting a response (data) from the device, is the host about to send some data to the device or is this a request with no data payload? The SETUP stage conveys this information.

The nRF52840 USBD peripheral will parse this header and store its contents in the following registers: BMREQUESTTYPE, BREQUEST, WVALUE{L,H}, WINDEX{L,H} and WLENGTH{L,H}. These registers match the logical division of the setup packet, in *fields*, as specified in the USB 2.0 specification. The fields that start with the letter 'b' are 1-byte large; the ones that start with the letter 'w' are 2-bytes large.

Open the `src/bin/rtic-usb-2.rs` file. 

The task here is to write a SETUP packet parser in the `common/usb` crate and reach the GOAL comment when executing the program. The starter code has already read the relevant registers using helper functions.

To implement the parser, refer to table 9-3 under section 9.4 of the USB 2.0 specification (page 250). Note that at this stage you only need to be able to parse the `GetDescriptor` variant of the `Request` enum and within that variant you only need to handle the `Device` variant of the `Descriptor` enum.

You should develop this part completely on the host. Switch the workspace to the `common/usb` directory and run the unit tests on the host using Rust Analyzer's "Test" button.

## Device descriptor

After receiving a GET_DESCRIPTOR request during the SETUP stage the device needs to respond with a descriptor during the DATA stage.

Open the `src/bin/rtic-usb-3.rs` file. 

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

Open the `src/bin/rtic-usb-4.rs` file. 

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
