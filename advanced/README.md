# `advanced`

> Advanced workshop

In this workshop we'll build a toy USB device application that gets enumerated and configured by the host.

The embedded application will run in a fully event driven fashion: only doing work when the host asks for it.

## The nRF52840

Some details about the nRF52840 microcontroller that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

## The nRF52840 Development Kit

The development board we'll use has two USB ports: J2 and J3 -- you can find a description of the board in the top-level README of this repository -- and an on-board J-Link programmer / debugger. USB port J2 is the J-Link's USB port. USB port J3 is the nRF52840's USB port. Connect the Development Kit to your computer using both ports.

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

The goal of this exercise is to get the nRF52840 SoC to show in this list. The embedded application will use the vendor ID (VID) and product ID (PID) defined in `advanced/common/consts`; the `usb-list` tool will highlight the USB device that matches that VID PID pair.

``` console
$ # expected output
$ cargo run
Bus 002 Device 001: ID 1d6b:0003
Bus 001 Device 002: ID 0cf3:e300
Bus 001 Device 003: ID 0c45:6713
Bus 001 Device 001: ID 1d6b:0002
Bus 001 Device 059: ID 2020:0717 <- nRF52840 on the nRF52840 Development Kit
```

## Hello, world!

First, open the `tools/dk-run` folder and run `cargo install --path . -f` to install the `dk-run` tool.

Next open the `advanced/firmware` folder in VS Code and open the `src/bin/hello.rs` file from the `advanced/apps` folder.

> Note: To ensure full Rust-Analyzer support, do not open the whole `embedded-trainings-2020` folder.

Give Rust Analyzer some time to analyze the file and its dependency graph. When it's done, a "Run" button will appear over the `main` function. If it doesn't appear on its own, type something in the file, delete and save. This should trigger a re-load.

Click the "Run" button to run the application on the microcontroller.

If you are not using VS code run the `cargo run --bin hello` command from the `advanced/firmware` folder.

> NOTE if you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear

The `firmware` workspace has been configured to cross-compile applications to the ARM Cortex-M architecture and then run them through the `dk-run` custom Cargo runner. The `dk-run` tool will load and run the embedded application on the microcontroller and collect logs from the microcontroller.

The `dk-run` process will terminate when the microcontroller enters the "halted" state. From the embedded application, one can enter the "halted" state using the `asm::bkpt` function. For convenience, an `exit` function is provided in the `dk` Hardware Abstraction Layer (HAL). This function is divergent like `std::process::exit` (`fn() -> !`) and can be used to halt the device and terminate the `dk-run` process.

Note that when the `dk-run` tool sees the device enter the halted state it will proceed to *reset-halt* the device. This is particularly important when writing USB applications because simply leaving the device in a halted state will make it appear as an unresponsive USB device to the host. Some OSes (e.g. Linux) will try to make an unresponsive device respond by power cycling the entire USB bus -- this will cause all other USB devices on the bus to be re-enumerated. Reset-halting the device will cause it to be electrically disconnected from the host USB bus and avoid the "power cycle the whole USB bus" scenario.

## Checking the API documentation

We'll be using the `dk` Hardware Abstraction Layer. It's good to have its API documentation handy. You can generate the documentation for that crate from the command line:

``` console
$ cargo doc -p dk --open
```

Run this command from within the `advanced/firmware` folder. It will open the generated documentation in your default web browser.

> NOTE if you are using Safari and the documentation is hard to read due to missing CSS, try opening it in a different browser.

## RTIC hello

RTIC, Real Time on Integrated Circuits, is a framework for building evented, time sensitive applications.

Open the `src/bin/rtic-hello.rs` file.

RTIC applications are written in RTIC's Domain Specific Language (DSL). The DSL extends Rust syntax with custom attributes like `#[init]` and `#[idle]`.

RTIC makes a clearer distinction between the application's initialization phase, the `#[init]` function, and the application's main loop or main logic, the `#[idle]` function. The initialization phase runs with interrupts disabled and interrupts are re-enabled before the `idle` function is executed.

`rtic::app` is a procedural macro that generates extra Rust code, in addition to the user's functions. The fully expanded version of the macro can be found in the file `target/rtic-expansion.rs`. This file will contain the expansion of the procedural macro for the last compiled RTIC application.

If you build the `rtic-hello` example and look at the generated `rtic-expansion.rs` file you can confirm that interrupts are disabled during the execution of the `init` function:

``` rust
fn main() -> ! {
    rtic::export::interrupt::disable();
    let late = init(init::Context::new(/* .. */));
    rtic::export::interrupt::enable();
    idle(idle::Context::new(/* .. */))
}
```

## Dealing with registers

Open the `src/bin/events.rs` file.

In this and the next section we'll look into RTIC's event handling features. To explore these features we'll use the action of connecting a USB cable to the DK's port J2 as the event we'd like to handle.

The example application enables the signaling of this "USB power" event in the `init` function. This is done using the low level register API generated by the [`svd2rust`] tool. The register API was generated from a SVD (System View Description) file, a file that describes all the peripherals and registers, and their memory layout, on a device. In our case the device was the nRF52840; a sample SVD file for this microcontroller can be found [here][nrf52840.svd].

[`svd2rust`]: https://crates.io/crates/svd2rust
[nrf52840.svd]: https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/nrf52840.svd

In the `svd2rust` API, peripherals are represented as structs. The fields of each peripheral struct are the registers associated to that peripheral. Each register field exposes methods to `read` and `write` to the register in a *single* memory operation.

The `read` and `write` methods take closure arguments. These closures in turn grant access to a "constructor" value, usually named `r` or `w`, which provides methods to modify the bitfields of a register. At the same time the API of these "constructors" prevent you from modifying the reserved parts of the register: you cannot write arbitrary values into registers; you can only write valid values into registers.

Apart from the `read` and `write` methods there's a `modify` method that performs a read-modify-write operation on the register; this API is also closure-based. The `svd2rust`-generated API is documented in detail in the `svd2rust` crate starting at [the Peripheral API section][svd2rust-api].

[svd2rust-api]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api

In Cortex-M devices interrupt handling needs to be enabled on two sides: on the peripheral side and on the core side. The register operations done in `init` take care of the peripheral side. The core side of the operation involves writing to the registers of the Nested Vector Interrupt Controller (NVIC) peripheral. This second part doesn't need to be done by the user in RTIC applications because the framework takes care of it.

## Event handling

Below the `idle` function you'll see a `#[task]` handler, a function. This *task* is bound to the POWER_CLOCK interrupt signal and will be executed, function-call style, every time the interrupt signal is raised by the hardware.

"Run" the `events` application. Then connect a micro-USB cable to your PC/laptop then connect the other end to the DK (port J3). You'll see the "POWER event occurred" message after the cable is connected.

Note that all tasks will be prioritized over the `idle` function so the execution of `idle` will be interrupted (paused) by the `on_power_event` task. When the `on_power_event` task finishes (returns) the execution of the `idle` will be resumed. This will become more obvious in the next section.

Try this: add an infinite loop to the end of `init` so that it never returns. Now run the program and connect the USB cable. What behavior do you observe? How would you explain this behavior? (hint: look at the `rtic-expansion.rs` file: under what conditions is the `init` function executed?)

## Task state

Now let's say we want to change the previous program to count how many times the USB cable (port J3) has been connected and disconnected.

Tasks run from start to finish, like functions, in response to events. To preserve some state between the different executions of a task we can add a *resource* to the task. In RTIC, resources are the mechanism used to *share* data between different tasks in a memory safe manner but they can also be used to hold task state.

To get the desired behavior we'll want to store some counter in the state of the `on_power_event` task.

Open the `src/bin/resource.rs` file. The starter code shows the syntax to declare a resource, the `Resources` struct, and the syntax to associate a resource to a task, the `resources` list in the `#[task]` attribute.

In the starter code a resource is used to *move* (by value) the POWER peripheral from `init` to the `on_power_event` task. The POWER peripheral then becomes part of the state of the `on_power_event` task and can be persistently accessed throughout calls to `on_power_event()` through a *reference*. The resources of a task are available via the `Context` argument of the task.

To elaborate more on this *move* action: in the `svd2rust` API, peripheral types like `POWER` are *singletons* (only a single instance of the type can ever exist). The consequence of this design is that holding a peripheral instance, like `POWER`, *by value* means that the function (or task) has exclusive access, or ownership, over the peripheral. This is the case of the `init` function: it owns the `POWER` peripheral but then transfers ownership over it to a task using the resource initialization mechanism.

We have moved the POWER peripheral into the task because we want to clear the USBDETECTED interrupt flag after it has been set by the hardware. If we miss this step the `on_power_event` task (function) will be called again once it returns and then again and again and again (ad infinitum).

Also note that in the starter code the `idle` function has been modified. Pay attention to the logs when you run the starter code.

Your task in this section will be to modify the program so that it prints the number of times the USB cable has been connected to the DK every time the cable is connected, as shown below.

``` console
(..)
INFO:resource -- on_power_event: cable connected 1 time
(..)
INFO:resource -- on_power_event: cable connected 2 times
(..)
INFO:resource -- on_power_event: cable connected 3 times
```

You can find a solution to this exercise in the `resource-solution.rs` file.

## USB enumeration

The USB protocol is complex so we'll leave out many details and focus only on the concepts required to get enumeration and configuration working. There are also several USB specific terms so we recommend checking chapter 2, "Terms and Abbreviations", of the USB specification (linked at the bottom of this document) every now and then.

So what is enumeration? A USB device, like the nRF52840, can be one of these three states: the Default state, the Address state or the Configured state. After being powered the device will start in the Default state. The enumeration process will take the device from the Default state to the Address state. As a result of the enumeration process the device will be assigned an address, in the range `1..=127`, by the host.

Each OS may perform the enumeration process slightly differently but the process will always involve these host actions:

- USB reset. This will put the device in the Default state, regardless of what state it was in.
- GET_DESCRIPTOR request to get the device descriptor.
- SET_ADDRESS request to assign an address to the device.

These host actions will be perceived as *events* by the nRF52840. During this workshop, we will gradually parse and handle these events and learn more about Embedded Rust along the way.

There are more USB concepts involved that we'll need to cover like descriptors, configurations, interfaces and endpoints but for now let's see how to handle USB events.

For each step of the course, we've prepared a `usb-<n>.rs` file that gives you a base structure and hints on how to proceed. The matching `usb-<n>-solution.rs` contains a sample solution should you need it. Switch from `usb-<n>.rs` to `usb-<n+1>.rs` when instructed and continue working from there.


## USB-1: Dealing with USB events

The USBD peripheral on the nRF52840 contains a series of registers, called EVENTS registers, that indicate the reason for entering the USBD event handler. These events must be handled by the application to complete the enumeration process.

Open the `firmware/src/bin/usb-1.rs` file. In this starter code the USBD peripheral is initialized in `init` and a task, named `main`, is bound to the interrupt signal USBD. This task will be called every time a new USBD event needs to be handled. The `main` task uses `usbd::next_event()` to check all the event registers; if any event is set (occurred) then the function returns the event, represented by the `Event` enum, wrapped in the `Some` variant. This `Event` is then passed to the `on_event` function for further processing.

Connect the USB cable to the port J3 then run the starter code.

In this section as a warm-up exercise you'll need to deal with the following USB events until you reach the EP0SETUP event.

- `USBRESET`. This event indicates that the host issued a USB reset signal. According to the USB specification this will move the device from any state to the `Default` state. Since we are currently not dealing with any other state, you can handle this state by doing nothing.

- `EP0SETUP`. The USBD peripheral has detected the SETUP stage of a control transfer. If you get to this point move to the next section.

- `EP0DATADONE`. The USBD peripheral is signaling the end of the DATA stage of a control transfer. You won't encounter this event just yet.

When you are done you should see this output:

``` console
(..)
INFO:usb_1 -- USB: UsbEp0Setup
INFO:usb_1 -- goal reached; move to the next section
```

Do not overthink this exercise; it is not a trick question. There is very little to do and no new functionality to add.

You can find the solution in the `usb-1-solution.rs` file.

Before we continue we need to discuss how data transfers work under the USB protocol.

## USB Endpoints

Under the USB protocol data transfers occur over *endpoints*.

Endpoints are similar to UDP or TCP ports in that they allow logical multiplexing of data over a single physical USB bus. USB endpoints, however, have directions: an endpoint can either be an IN endpoint or an OUT endpoint. The direction is always from the perspective of the host so at an IN endpoint data travels from the device to the host and at an OUT endpoint data travels from the host to the device.

Endpoints are identified by their address, a zero-based index, and direction. There are four types of endpoints: control endpoints, bulk endpoints, interrupt endpoints and isochronous endpoints. Each endpoint type has different properties: reliability, latency, etc. In this workshop we'll only need to deal with control endpoints.

All USB devices must use "endpoint 0" as the default control endpoint. "Endpoint 0" actually refers to two endpoints: endpoint 0 IN and endpoint 0 OUT. This endpoint pair is used to establish a *control pipe*, a bidirectional communication channel between the host and device where data is exchanged using a predefined format. The default control pipe over endpoint 0 is mandatory: it must always be present and must always be active.

For detailed information about endpoints check section 5.3.1, Device Endpoints, of the [USB 2.0 specification][usb20].

[usb20]: https://www.usb.org/document-library/usb-20-specification

## Control transfers

The control pipe handles *control transfers*, a special kind of data transfer used by the host to issue *requests*. A control transfer is a data transfer that occurs in three stages: a SETUP stage, an optional DATA stage and a STATUS stage.

During the SETUP stage the host sends 8 bytes of data that identify the control request. Depending on the issued request there may be a DATA stage or not; during the DATA stage data is transferred either from the device to the host or the other way around. During the STATUS stage the device acknowledges, or not, the whole control request.

For detailed information about control transfers check section 5.5, Control Transfers, of the [USB 2.0 specification][usb20].

## USB-2: SETUP stage

At the end of program `usb-1` we received a EP0SETUP event. This event signals the *end* of the SETUP stage of a control transfer.  The nRF52840 USBD peripheral will automatically receive the SETUP data and store it in the following registers: BMREQUESTTYPE, BREQUEST, WVALUE{L,H}, WINDEX{L,H} and WLENGTH{L,H}. These registers are documented in sections 6.35.13.31 to 6.35.13.38 of the [nRF52840 Product Specification][nrf product spec].

[nrf product spec]: https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf

The format of this setup data is documented in section 9.3 of the USB specification. Your next task is to parse it. We will start with the GET_DESCRIPTOR request, which is described in detail in section 9.4.3 of the USB specification. All the constants you will need are described in Tables 9-3, 9-4 and 9-5.

> NOTE: If you'd like to learn more, take a look at Section 9.4, Standard Descriptor Requests, of the USB specification.

When you need to write some `no_std` code that does not involve device-specific I/O you should consider writing it as a separate crate. This way, you can test it on your development machine (e.g. `x86_64`) using the standard `cargo test` functionality.

So that's what we'll do here. In `advanced/common/usb/lib.rs` you'll find starter code for writing a `no_std` SETUP data parser. The starter code contains some unit tests; you can run them with `cargo test` (from within the `usb` folder) or you can use Rust Analyzer's "Test" button in VS code.

The definition of `Descriptor::Configuration` as well as the associated test has been "commented out" using an `#[cfg(TODO)]` attribute because it is not handled by the firmware yet. Delete the `#[cfg(TODO)]` so that the unit tests can access it. This pattern is used for enum members and test functions throughout this workshop, so keep it in mind should you see it again.

Now, proceed as follows:

1. **Parse GET_DESCRIPTOR requests:**  
Modify `Request::parse()` in `advanced/common/usb` to recognize a GET_DESCRIPTOR request so that the `get_descriptor_device` test passes. Note that the parser already handles SET_ADDRESS requests.
    - check table 9-4 in the USB specification for Request Codes
    - remember that you can define binary literals by prefixing them with `0b`
    - you can use bit shifts (`>>`) and casts (`as u8`) to get the high/low bytes of a `u16`

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

## Device descriptor

After receiving a GET_DESCRIPTOR request during the SETUP stage the device needs to respond with a *descriptor* during the DATA stage.

A descriptor is a binary encoded data structure sent by the device to the host. The device descriptor, in particular, contains information about the device, like its product and vendor identifiers and how many *configurations* it has. The format of the device descriptor is specified in section 9.6.1, Device, of the USB specification.

As far as the enumeration process goes, the most relevant fields of the device descriptor are the number of configurations and `bcdUSB`, the version of the USB specification the devices adheres to. In `bcdUSB` you should report compatibility with USB 2.0.

What about (the number of) configurations?

A *configuration* is akin to an operation mode. USB devices usually have a single configuration that will be the only mode in which they'll operate, for example a USB mouse will always act as a USB mouse. Some devices, though, may provide a second configuration for the purpose of firmware upgrades. For example a printer may enter DFU (Device Firmware Upgrade) mode, a second *configuration*, so that a user can update its firmware; while in DFU mode the printer will not provide printing functionality.

The specification mandates that a device must have at least one available configuration so we can report a single configuration in the device descriptor.

## USB-3: DATA stage

The next step is to respond to the GET_DESCRIPTOR request with a device descriptor. To do this we'll use the `dk::usb::Ep0In` abstraction -- we'll look into what the abstraction does in a future section; for now we'll just use it.

An instance of this abstraction is available in the `board` value (`#[init]` function). The first step is to make this `Ep0In` instance available to the `on_event` function.

The `Ep0In` API has two methods: `start` and `end` (also see their API documentation). `start` is used to start a DATA stage; this method takes a *slice of bytes* (`[u8]`) as argument; this argument is the response data. The `end` method needs to be called after `start`, when the EP0DATADONE event is raised, to complete the control transfer. `Ep0In` will automatically issue the STATUS stage that must follow the DATA stage.

To implement responding to a GET_DESCRIPTOR request, do the following:

1. **Extend the parser implementation to handle a GET_DESCRIPTOR request:** make the `common/usb/lib.rs::get_descriptor_configuration()` test run successfully.
2. **Answer the Descriptor Request:** extend `usb-3.rs` so that it uses `Ep0In` to respond to the `GET_DESCRIPTOR Device` request (and only to that request). The response must be a device descriptor with its fields set to these values:

- `bDeviceClass = bDeviceSubClass = bDeviceProtocol = 0`, these are unimportant for enumeration
- `bMaxPacketSize0 = 64`, this is the most performant option (minimizes exchanges between the device and the host) and it's assumed by the `Ep0In` abstraction
- `idVendor = consts::VID`, value expected by the `usb-list` tool (\*)
- `idProduct = consts::PID`, value expected by the `usb-list` tool (\*)
- `bcdDevice = 0x0100`, this means version 1.0 but any value should do
- `iManufacturer = iProduct = iSerialNumber = None`, string descriptors not supported
- `bNumConfigurations = 1`, must be at least `1` so this is the minimum value

>(\*) the `common` crate refers to the crate in the `advanced/common` folder. It is already part of the `firmware` crate dependencies.

Although you can create the device descriptor by hand as an array filled with magic values we *strongly* recommend you use the `usb2::device::Descriptor` abstraction. The crate is already in the dependency list of the project; you can open its API documentation with the following command: `cargo doc -p usb2 --open`.

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

## DMA (Direct Memory Access)

Let's zoom into the `Ep0In` abstraction used in `usb4.rs` next. You can use VSCode's "Go to Definition" to see the implementation of the `Ep0In.start()` method. What this method does is start a DMA transfer to send `bytes` to the host. The data (`bytes`) is first copied into an internal buffer and then the DMA is configured to move the data from that internal buffer to the USBD peripheral.

The signature of the `start()` method does *not* ensure that:

- `bytes` won't be deallocated before the DMA transfer is over (e.g. `bytes` could be pointing into the stack), or that
- `bytes` won't be modified right after the DMA transfer starts (this would be a data race in the general case).

For these two safety reasons the API is implemented using an internal buffer called `buffer`. The internal buffer has a `'static` lifetime so it's guaranteed to never be deallocated -- this prevents issue (a). The `busy` flag prevents any further modification to the internal buffer -- from the public API -- while the DMA transfer is in progress.

Apart from thinking about lifetimes and explicit data races in the surface API one must internally use memory fences to prevent reordering of memory operations (e.g. by the compiler), which can also cause data races. DMA transfers run in parallel to the instructions performed by the processor and are "invisible" to the compiler.

In the implementation of the `start` method, data is copied from `bytes` to the internal buffer (a `memcpy()` operation) and then the DMA transfer is started with a write to the `TASKS_STARTEPIN0` register. The compiler sees the start of the DMA transfer (register write) as an unrelated memory operation so it may move the `memcpy()` to *after* the DMA transfer has started. This reordering results in a data race: the processor modifies the internal buffer while the DMA is reading data out from it.

To avoid this reordering a memory fence, `dma_start()`, is used. The fence pairs with the *store* operation (register write) that starts the DMA transfer and prevents the previous `memcpy()`, and any other memory operation, from being move to *after* the store operation.

Another memory fence, `dma_end()`, is needed at the end of the DMA transfer. In the general case, this prevents instruction reordering that would result in the processor accessing the internal buffer *before* the DMA transfer has finished. This is particularly problematic with DMA transfers that modify a region of memory which the processor intends to read after the transfer.

> Note: Not relevant to the DMA operation but relevant to the USB specification, the `start()` method sets a shortcut in the USBD peripheral to issue a STATUS stage right after the DATA stage is finished. Thanks to this it is not necessary to manually start a STATUS stage after calling the `end` method.

## USB-4: Supporting more standard requests

After responding to the `GET_DESCRIPTOR Device` request the host will start sending different requests. The parser in `common/usb` will need to be updated to handle these requests:

1. `GET_DESCRIPTOR Configuration`, see section 9.4.3 of the USB spec
2. `SET_CONFIGURATION`, see section 9.4.7 of the USB spec -- this request is likely to only be observed on Linux during enumeration

The starter `common/usb` code contains unit tests for these other requests as well as extra `Request` variants for these requests. All of them have been commented out using a `#[cfg(TODO)]` attribute which you can remove once you need any new variant or new unit test.

For each green test, you can extend `usb-4.rs` to handle the new requests your parser is now able to recognize. **Make sure to read the next sections as you're working**, since they contain explanations about the concepts used and needed to complete this task.

If you need a reference, you can find solutions to parsing `GET_DESCRIPTOR Configuration` and `SET_CONFIGURATION` requests in the following files:

- `advanced/common/src/get-descriptor-configuration.rs`
- `advanced/common/src/set-configuration.rs`

Each file contains just enough code to parse the request in its name and the `GET_DESCRIPTOR Device` request. So you can refer to `set-configuration.rs` without getting "spoiled" about how to parse the `SET_CONFIGURATION` request.

## Error handling in embedded Rust

Since the logic of the `EP0SETUP` event handling is getting more complex with each added event, you can see that `usb-4.rs` was refactored to add error handling: the event handling now happens in a separate function *that returns a `Result`*. When it encounters an invalid host request, it returns the `Err` variant which can be handled by stalling the endpoint:

``` rust
fn on_event(/* parameters */) {
    match event {
        /* ... */
        Event::UsbEp0Setup => {
            if ep0setup(/* arguments */).is_err() {
                // unsupported or invalid request:
                // TODO add code to stall the endpoint
                log::warn!("EP0: unexpected request; stalling the endpoint");
            }
        }
    }
}

fn ep0setup(/* parameters */) -> Result<(), ()> {
    let req = Request::parse(/* arguments_*/)?;
    //                                       ^ early returns an `Err` if it occurs

    // TODO respond to the `req`; return `Err` if the request was invalid in this state

    Ok(())
}
```

Note that there's a difference between the error handling done here and the error handling commonly done in `std` programs. `std` programs usually bubble up errors to the top `main` function (using the `?` operator), report the error (or chain of errors) and then exit the application with a non-zero exit code. This approach is usually not appropriate for embedded programs as  
(1) `main` cannot return,  
(2) there may not be a console to print the error to and/or  
(3) stopping the program, and e.g. requiring the user to reset it to make it work again, may not be desirable behavior.  
For these reasons in embedded software errors tend to be handled as early as possible rather than propagated all the way up.

This does not preclude error *reporting*. The above snippet includes error reporting in the form of a `log::warn!` statement. This log statement may not be included in the final release of the program as it may not be useful, or even visible, to an end user but it is useful during development.

## Updating Device state

At some point during the initialization you'll receive a `SET_ADDRESS` request that will move the device from the `Default` state to the `Address` state. If you are working on Linux, you'll also receive a `SET_CONFIGURATION` request that will move the device from the `Address` state to the `Configured` state. Additionally, some requests are only valid in certain states– for example `SET_CONFIGURATION` is only valid if the device is in the `Address` state. For this reason `usb-4.rs` will need to keep track of the device's current state.

The device state should be tracked using a resource so that it's preserved across multiple executions of the `USBD` event handler. The `usb2` crate has a `State` enum with the 3 possible USB states: `Default`, `Address` and `Configured`. You can use that enum or roll your own.

Start tracking and updating the device state to move your request handling forward:

1. **Update the handling of the `USBRESET` event:** Instead of ignoring it, we now want it to change the state of the USB device. See section 9.1 USB Device States of the USB specification for details on what to do.

2. **Update the handling of `SET_ADDRESS` requests:** See the section on [Handling SET_ADDRESS Requests](#handling-set_address-requests) of this tutorial for details.

3. **Implement the handling of `GET_DESCRIPTOR Configuration` requests:** See the section on [Handling GET_DESCRIPTOR Configuration Requests](#handling-get_descriptor-configuration-requests) of this tutorial for details.

## Dealing with unknown requests: Stalling the endpoint

You may come across host requests other than the ones listed in previous sections.

For this situation, the USB specification defines a device-side procedure for "stalling an endpoint", which amounts to the device telling the host that it doesn't support some request.
> This procedure should be used to deal with invalid requests, requests whose SETUP stage doesn't match any USB 2.0 standard request, and requests not supported by the device– for instance the SET_DESCRIPTOR request is not mandatory.

You can use the `dk::usbd::ep0stall()` helper function to stall endpoint 0.
Your task is to do this in the right place in `usb-4.rs`.

## Handling SET_ADDRESS Requests

> This request should come right after the `GET_DESCRIPTOR Device` request if you're using Linux, or be the first request sent to the device by Mac OS.

Section 9.4.6, Set Address, describes how to handle this request but below you can find a summary:

- If the device is in the `Default` state, then
  - if the requested address was `0` (`None` in the `usb` API) then the device should stay in the `Default` state
  - otherwise the device should move to the `Address` state

- If the device is in the `Address` state, then
  - if the requested address was `0` (`None` in the `usb` API) then the device should return to the `Default` state
  - otherwise the device should remain in the `Address` state but start using the new address

- If the device is in the `Configured` state this request results in "unspecified" behavior according to the USB specification. You should stall the endpoint in this case.

> Note: According to the USB specification the device needs to respond to this request with a STATUS stage -- the DATA stage is omitted. The nRF52840 USBD peripheral will automatically issue the STATUS stage and switch to listening to the requested address (see the USBADDR register) so no interaction with the USBD peripheral is required for this request.
>
> For more details, read the introduction of section 6.35.9 of the nRF52840 Product Specification 1.0 (pages 486 and 487).


## Handling GET_DESCRIPTOR Configuration Requests

When the host issues a GET_DESCRIPTOR request the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

We have covered configurations and endpoints but what is an *interface*?

### Interface

An interface is closest to a USB device's function. For example, a USB mouse may expose a single HID (Human Interface Device) interface to report user input to the host. USB devices can expose multiple interfaces within a configuration. For example, the nRF52840 Dongle could expose both a CDC ACM interface (AKA virtual serial port) *and* a HID interface; the first interface could be used for (`log::info!`-style) logs; and the second one could provide a RPC (Remote Procedure Call) interface to the host for controlling the nRF52840's radio.

An interface is made up of one or more *endpoints*. To give an example, a HID interface can use two (interrupt) endpoints, one IN and one OUT, for bidirectional communication with the host. A single endpoint cannot be used by more than one interface with the exception of the special "endpoint 0", which can be (and usually is) shared by all interfaces.

For detailed information about interfaces check section 9.6.5, Interface, of the USB specification.

### Configuration descriptor

The configuration descriptor describes one of the device configurations to the host. The descriptor contains the following information about a particular configuration:

- the total length of the configuration: this is the number of bytes required to transfer this configuration descriptor and the interface and endpoint descriptors associated to it
- its number of interfaces -- must be >= 1
- its configuration value -- this is *not* an index and can be any non-zero value
- whether the configuration is self-powered
- whether the configuration supports remote wakeup
- its maximum power consumption

> The full format of the configuration descriptor is specified in section 9.6.3, Configuration, of the USB specification.

### Interface descriptor

The interface descriptor describes one of the device interfaces to the host. The descriptor contains the following information about a particular interface:

- its interface number -- this is a zero-based index
- its alternate setting -- this allows configuring the interface
- its number of endpoints
- class, subclass and protocol -- these define the interface (HID, or TTY ACM, or DFU, etc.) according to the USB specification

The number of endpoints can be zero and endpoint zero must not be accounted when counting endpoints.
> The full format of the interface descriptor is specified in section 9.6.5, Interface, of the USB specification.

### Endpoint descriptor

We will not need to deal with endpoint descriptors in this workshop but they are specified in section 9.6.6, Endpoint, of the USB specification.

### Response

So how should we respond to the host? As our only goal is to be enumerated we'll respond with the minimum amount of information possible.

**First, check the request:**  
Configuration descriptors are requested by *index*, not by their configuration value. Since we reported a single configuration in our device descriptor the index in the request must be zero. Any other value should be rejected by stalling the endpoint (see section [Dealing with unknown requests: Stalling the endpoint](#dealing-with-unknown-requests-stalling-the-endpoint) for more information).

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

## SET_CONFIGURATION (Linux & Mac OS)

On Linux and Mac OS, the host will likely send a SET_CONFIGURATION request right after enumeration to put the device in the `Configured` state. For now you can stall the request. It is not necessary at this stage because the device has already been enumerated.

## Idle state

Once you have handled all the previously covered requests the device should be enumerated and remain idle awaiting for a new host request. Your logs may look like this:

``` console
INFO:usb_4 -- USB: UsbReset @ 318.66455ms
INFO:usb_4 -- USB reset condition detected
INFO:usb_4 -- USB: UsbEp0Setup @ 391.418456ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Device, length: 64 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 391.723632ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbReset @ 442.016601ms
INFO:usb_4 -- USB reset condition detected
INFO:usb_4 -- USB: UsbEp0Setup @ 514.709471ms
INFO:usb_4 -- EP0: SetAddress { address: Some(17) }
INFO:usb_4 -- USB: UsbEp0Setup @ 531.37207ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Device, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 531.646727ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 531.829832ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 532.226562ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 532.592772ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
INFO:usb_4 -- USB: UsbEp0Setup @ 533.020018ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 9 }
INFO:dk::usbd -- EP0IN: start 9B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 533.386228ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 533.569335ms
INFO:usb_4 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_4 -- USB: UsbEp0DataDone @ 533.935546ms
INFO:usb_4 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_4 -- USB: UsbEp0Setup @ 534.118651ms
INFO:usb_4 -- EP0: SetConfiguration { value: Some(42) }
ERROR:usb_4 -- EP0IN: unexpected request; stalling the endpoint
```

Note that these logs are from a Linux host where a `SET_CONFIGURATION` request is sent after the `SET_ADDRESS` request. On other OSes you may not get that request before the bus goes idle. Also note that there are some `GET_DESCRIPTOR DeviceQualifier` requests in this case; you do not need to parse them in the `usb` crate as they'll be rejected (stalled) anyways.

You can find traces for other OSes in these files (they are next to this README):

- `win-enumeration.txt`
- `macos-enumeration.txt`

At this point you can double check that the enumeration works by running the [`usb-list` tool](#listing-usb-devices) while `usb-4.rs` is running.

``` console
Bus 001 Device 013: ID 1366:1015 <- J-Link on the nRF52840 Development Kit
(..)
Bus 001 Device 016: ID 2020:0717 <- nRF52840 on the nRF52840 Development Kit
```

Both the J-Link and the nRF52840 should appear in the list.

You can find a working solution up to this point in `src/bin/usb-4-solution.rs`. Note that the solution uses the `usb2` crate to parse SETUP packets and that crate supports parsing all standard requests.

## Inspecting the descriptors

There's a tool in the `advanced/host/` folder called `print-descs`. You can run this tool to print all the descriptors reported by your application.

``` console
$ print-descs
DeviceDescriptor {
    bLength: 18,
    bDescriptorType: 1,
    bcdUSB: 512,
    bDeviceClass: 0,
    bDeviceSubClass: 0,
    bDeviceProtocol: 0,
    bMaxPacketSize: 64,
    idVendor: 8224,
    idProduct: 1815,
    bcdDevice: 256,
    iManufacturer: 0,
    iProduct: 0,
    iSerialNumber: 0,
    bNumConfigurations: 1,
}
address: 22
config0: ConfigDescriptor {
    bLength: 9,
    bDescriptorType: 2,
    wTotalLength: 18,
    bNumInterfaces: 1,
    bConfigurationValue: 42,
    iConfiguration: 0,
    bmAttributes: 192,
    bMaxPower: 250,
    extra: None,
}
iface0: [
    InterfaceDescriptor {
        bLength: 9,
        bDescriptorType: 4,
        bInterfaceNumber: 0,
        bAlternateSetting: 0,
        bNumEndpoints: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        iInterface: 0,
    },
]
```

The output above corresponds to the descriptor values we suggested. If you used different values, e.g. for `bMaxPower`, you'll a slightly different output.

## Getting it configured

At this stage the device will be in the `Address` stage. It has been identified and enumerated by the host but cannot yet be used by host applications. The device must first move to the `Configured` state before the host can start, for example, HID communication or send non-standard requests over the control endpoint.

Windows and macOS will enumerate the device but not automatically configure it after enumeration. Here's what you should do to force the host to configure the device.

### Linux and Mac OS

Nothing extra needs to be done if you're working on a Linux or Mac OS host. The host will automatically send a `SET_CONFIGURATION` request so proceed to the `SET_CONFIGURATION` section to see how to handle the request.

### Windows

After getting the device enumerated and into the idle state, open the Zadig tool (covered in the setup instructions; see the top README) and use it to associate the nRF52840 USB device to the WinUSB driver. The nRF52840 will appear as a "unknown device" with a VID and PID that matches the ones defined in the `common` crate

Now modify the `print-descs` program to "open" the device -- this operation is commented out in the source code. With this modification `print-descs` will cause Windows to send a `SET_CONFIGURATION` request to configure the device. You'll need to run `print-descs` to test out the correct handling of the `SET_CONFIGURATION` request.

### SET_CONFIGURATION

Section 9.4.7, Set Configuration, of the USB spec describes how to handle this request but below you can find a summary:

- If the device is in the `Default` state, you should stall the endpoint because the operation is not permitted in that state.

- If the device is in the `Address` state, then
  - if the requested configuration value is 0 (`None` in the `usb` API) then stay in the `Address` state
  - if the requested configuration value is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state
  - if the requested configuration value is not valid then stall the endpoint

- If the device is in the `Configured` state, then
  - if the requested configuration value is 0 (`None` in the `usb` API) then return to the `Address` state
  - if the requested configuration value is non-zero and valid (was previously reported in a configuration descriptor) then move to the `Configured` state with the new configuration value
  - if the requested configuration value is not valid then stall the endpoint

In all the cases where you did not stall the endpoint (by returning `Err`) you'll need to acknowledge the request by starting a STATUS stage.  
This is done by writing 1 to the TASKS_EP0STATUS register.

NOTE: On Windows, you may get a `GET_STATUS` request *before* the `SET_CONFIGURATION` request and although you *should* respond to it, stalling the `GET_STATUS` request seems sufficient to get the device to the `Configured` state.

### Expected output

Once you are correctly handling the `SET_CONFIGURATION` request you should get logs like these:

``` console
INFO:usb_5 -- USB: UsbReset @ 397.15576ms
INFO:usb_5 -- USB reset condition detected
INFO:usb_5 -- USB: UsbEp0Setup @ 470.00122ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Device, length: 64 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 470.306395ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbReset @ 520.721433ms
INFO:usb_5 -- USB reset condition detected
INFO:usb_5 -- USB: UsbEp0Setup @ 593.292235ms
INFO:usb_5 -- EP0: SetAddress { address: Some(21) }
INFO:usb_5 -- USB: UsbEp0Setup @ 609.954832ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Device, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 610.260008ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 610.443113ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 610.809325ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 611.175535ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: DeviceQualifier, length: 10 }
WARN:usb_5 -- EP0IN: stalled
INFO:usb_5 -- USB: UsbEp0Setup @ 611.511228ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 9 }
INFO:dk::usbd -- EP0IN: start 9B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 611.846922ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 612.030027ms
INFO:usb_5 -- EP0: GetDescriptor { descriptor: Configuration { index: 0 }, length: 18 }
INFO:dk::usbd -- EP0IN: start 18B transfer
INFO:usb_5 -- USB: UsbEp0DataDone @ 612.365721ms
INFO:usb_5 -- EP0IN: transfer complete
INFO:dk::usbd -- EP0IN: transfer done
INFO:usb_5 -- USB: UsbEp0Setup @ 612.640378ms
INFO:usb_5 -- EP0: SetConfiguration { value: Some(42) }
INFO:usb_5 -- entering the configured state
```

These logs are from a Linux host. You can find traces for other OSes in these files (they are next to this README):

- `win-configured.txt`, this file only contains the logs produced by running `print-descs`
- `macos-configured.txt`

You can find a solution to this part of the exercise in `src/bin/usb-5-solution.rs`.

## Next steps

We have covered only a few of the core features of the RTIC framework but the framework has many more features like *software* tasks, tasks that can be spawned by the software; message passing between tasks; and task scheduling, which allows the creation of periodic tasks. We encourage to check the [RTIC book][rtic-book] which describes the features we haven't covered here.

[rtic-book]: https://rtic.rs/0.5/book/en/

[`usb-device`] is a library for building USB devices. It has been built using traits (the pillar of Rust's *generics*) such that USB interfaces like HID and TTY ACM can be implemented in a device agnostic manner. The device details then are limited to a trait *implementation*. There's a work in progress implementation of the `usb-device` trait for the nRF52840 device in [this PR] and there are many `usb-device` "classes" like [HID] and [TTY ACM] that can be used with that trait implementation.

[this PR]: https://github.com/nrf-rs/nrf-hal/pull/144
[HID]: https://crates.io/crates/usbd-hid
[TTY ACM]: https://crates.io/crates/usbd-serial

[`usb-device`]: https://crates.io/crates/usb-device

## String descriptors

> TODO more material if needed

## Custom control transfers

> TODO more material if needed

## References

- [nRF52840 Product Specification 1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
- [Universal Serial Bus Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)
