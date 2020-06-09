# `beginner`

> Beginner workshop

## Hardware 

In this workshop we'll use both the nRF52840 Development Kit (DK) and the nRF52840 Dongle. We'll mainly develop programs for the DK and use the Dongle to assist with some exercises.

For the span of this workshop keep the nRF52840 DK connected to your PC using a micro-USB cable. Connect the USB cable to the J2 port on the nRF52840 DK. Instructions to identify the USB ports on the nRF52840 board can be found in the top level README file. 

## The nRF52840

Some details about the nRF52840 microcontroller that are relevant to this workshop.

- single core ARM Cortex-M4 processor clocked at 64 MHz
- 1 MB of Flash (at address `0x0000_0000`)
- 256 KB of RAM (at address `0x2000_0000`)
- IEEE 802.15.4 and BLE (Bluetooth Low Energy) compatible radio
- USB controller (device function)

## Parts of an embedded program

1. Open the `beginner/apps` folder in VS Code.

``` console
$ # or use "File > Open Folder" in VS Code
$ code beginner/apps
```

2. Open the `src/bin/hello.rs` file.

Note the differences between this embedded program and a desktop program:

 The `#![no_std]` attribute indicates that the program will not make use of the standard library, `std` crate. Instead it will use the `core` library, a subset of the standard library that does not on a underlying operating system (OS).

The `#![no_main]` attribute indicates that the program will use a custom entry point instead of the default one: `fn main() { .. }`.

The `#[entry]` attribute declares the custom entry point of the program. The entry point must be a divergent function; note that the return type is the never type `!`. The function is not allowed to return; therefore the program is not allowed to terminate.

## Building the program

The following command cross compiles the program to the ARM Cortex-M4 architecture. The `--target` arguments instructs Cargo to cross compile the program.

``` console
$ cargo build --target thumbv7em-none-eabi --bin hello
```

The default in a new Cargo project is to compile for the host (native compilation). Within the `beginner/apps` folder you can however omit the `--target ` flag and Cargo will still cross compile for the ARM Cortex-M4 architecture.

``` console
$ cargo build --bin hello
```

The reason for this is that the default compilation target has been set to ARM Cortex-M4 in the Cargo configuration file (`.cargo/config`):

``` text
# .cargo/config
[build]
target = "thumbv7em-none-eabi"
```

The output of the compilation process will be an ELF (Executable and Linkable Format) file. The file will be placed in the `beginner/apps/target` directory. To display the amount of Flash the program will occupy on the target device use the `rust-size` tool (part of the `cargo-binutils` package):

``` console
$ rust-size target/thumbv7em-none-eabi/debug/hello
   text    data     bss     dec     hex filename
  14564       8    2124   16696    4138 target/thumbv7em-none-eabi/debug/hello
```

`14460` bytes is the amount of Flash memory the program will occupy.

Alternatively, you can run the `cargo-size` subcommand, which will build the program before displaying the size of the binary.

``` console
$ cargo size --bin hello
   text    data     bss     dec     hex filename
  14564       8    2124   16696    4138 hello
```

Passing the `-A` flag to `rust-size` or `cargo-size` will give a more detailed breakdown of the static memory usage:

``` console
$ # omit the `--` flag if using `rust-size`
$ cargo size --bin hello -- -A
hello  :
section              size        addr
.vector_table         256         0x0
.text                9740       0x100
.rodata              4568      0x270c
.data                   8  0x20000000
.bss                 2124  0x20000008
.uninit                 0  0x20000854
```

The `.vector_table` section contains the *vector table*, a data structure required by the Cortex-M ISA. The `.text` section contains the instructions the program will execute. The `.rodata` section contains constants like strings literals. These three sections are contiguously located in Flash memory -- Flash memory spans from address `0x0000_0000` to `0x0010_0000` (1 MB).

The next three sections, `.data`, `.bss` and `.uninit`, are located in RAM -- RAM memory spans the address range `0x2000_0000` - `0x2004_0000` (256 KB). These sections contain statically allocated variables (`static` variables).

## Flashing the program

The following command will flash the ELF file to the device.

``` console
$ cargo flash --chip nRF52840_xxAA --elf target/thumbv7em-none-eabi/debug/hello
```

Alternatively you can run this command, which builds the application before flashing it.

``` console
$ cargo flash --chip nRF52840_xxAA --bin hello
```

The `cargo-flash` subcommand flashes and runs the program but won't display logs. It is a deployment tool.

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, named J-Link, is connected to the nRF52840 through a electrical interface known as JTAG. The JTAG protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.

## Viewing logs

To observe the program logs you can use the `cargo-embed` tool.

``` console
$ cargo embed --bin hello
```

This command will bring up a Text User Interface (TUI). You should see "Hello, world!" in the output. You can close the interface using Ctrl-C.

`cargo-embed` has no `--chip` flag; instead the target chip needs to be specified in a file named `Embed.toml`. This file must be placed in the root of the Cargo project / workspace, next to the `Cargo.toml` file.

``` toml
# Embed.toml
[general]
chip = "nRF52840_xxAA"
```

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to overrun the ring buffer; this causes old log messages to be overwritten.

## Running the program from VS code

Both `cargo-embed` and `cargo-flash` are tools based on the `probe-rs` library. This library exposes an API to communicate with the J-Link and perform all the operations exposed by the JTAG protocol. For this workshop we have developed a small Cargo runner that uses the `probe-rs` library to streamline the process of running a program and printing logs, like `cargo-embed`, while also having better integration into VS code.

1. Browse to the `tools/dk-run` folder and run this command from there:

``` console
$ cargo install --path . -f
```

2. Open the `beginner/apps` folder in VS Code; then open the `src/bin/hello.rs` file and click the "Run" button that's hovering over the `main` function.

If you are not using VS code, run the command `cargo run --bin hello` from within the `beginer/apps` folder. Rust Analyzer's "Run" button is a short-cut for that command.

``` console
$ cargo run --bin hello
INFO:hello -- Hello, world!
stack backtrace:
   0: 0x0000229c - __bkpt
   1: 0x0000030e - hello::__cortex_m_rt_main
   2: 0x0000011a - main
   3: 0x00001ba2 - Reset
```

`cargo run` will compile the application and then invoke the `dk-run` tool with its argument set to the path of the output ELF file.

Unlike `cargo-embed`, `dk-run` will terminate when the program reaches a breakpoint (`asm::bkpt`) that halts the device. Before exiting `dk-run` will print a stack backtrace of the program starting from the breakpoint. This can be used to write small test programs that are meant to perform some work and then terminate.

## Panicking

Open the `beginner/apps` folder in VS Code then open the `src/bin/panic.rs` file and click the "Run" button.

This program attempts to index an array beyond its length and this results in a panic.

``` console
ERROR:panic_log -- panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:29:13
stack backtrace:
   0: 0x000022f0 - __bkpt
   1: 0x00002010 - rust_begin_unwind
   2: 0x00000338 - core::panicking::panic_fmt
   3: 0x00000216 - core::panicking::panic_bounds_check
   4: 0x0000016a - panic::bar
   5: 0x00000158 - panic::foo
   6: 0x00000192 - panic::__cortex_m_rt_main
   7: 0x00000178 - main
   8: 0x0000199e - Reset
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute. In the example, the *panic handler* is defined in the `panic_log` crate but we can also implement it manually: comment out the `panic_log` import and add the following function to the example:

``` rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    loop {
        asm::bkpt()
    }
}
```

Now run the program again. Try changing the format string of the `error!` macro.

## Using a Hardware Abstraction Layer (HAL)

In this section we'll start using the hardware features of the nRF52840 and the board.

Open the `beginner/apps` folder in VS Code then open the `src/bin/led.rs` file.

The `dk` crate / library is a Hardware Abstraction Layer (HAL) over the nRF52840 Development Kit. The purpose of a HAL is to abstract away the device-specific details of the hardware, for example registers, and instead expose a higher level API more suitable for application development.

The `dk::init` function we have been calling in all programs initializes a few of the nRF52840 peripherals and returns a `Board` structure that provides access to those peripherals. We'll first look at the `Leds` API. Open the documentation for the `dk` crate running the following command from the `beginner/apps` folder:

``` console
$ cargo doc -p dk --open
```

Check the API docs of the `Led` abstraction then run the `led` program. Two of the green LEDs on the board should turn on; the other two should stay off.

Now, uncomment the `log::set_max_level` line. This will make the logs more verbose; they will now include logs from the board initialization function (`dk::init`) and from the `Led` API.

Among the logs you'll find the line "I/O pins have been configured for digital output". At this point the electrical pins of the nRF52840 microcontroller has been configured to drive the 4 LEDs on the board.

After the `dk::init` logs you'll find logs about the `Led` API. As the logs indicate an LED becomes active when the output of the pin is a *logical zero*, which is also referred as the "low" state. This "active low" configuration does not apply to all boards: it depends on how the pins have been wired to the LEDs. You should refer to the [board documentation] to find out which pins are connected to LEDs and whether "active low" or "active high" applies to it.

[board documentation]: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fintro.html

## Timers and time

Next we'll look into the time related APIs exposed by the `dk` HAL.

Open the `beginner/apps` folder in VS Code then open the `src/bin/blinky.rs` file.

This program will blink (turn on and off) one of the LEDs on the board. The time interval between each toggle operation is one second. This wait time between consecutive operations is generated by the blocking `timer.wait` operation. This function call will block the program execution for the specified [`Duration`] argument.

[`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html

The other time related API exposed by the `dk` HAL is the `dk::uptime` function. This function returns the time that has elapsed since the call to the `dk::init` function. This function is used in the program to log the time of each LED toggle operation.

Next, we'll look into the radio API exposed by the `dk` HAL. But before that we'll need to set up the nRF52840 Dongle.

## nRF52840 Dongle

From this section on, we'll use the nRF52840 Dongle in addition to the nRF52840 DK. We'll run some pre-compiled programs on the Dongle and write programs for the DK that will interact with the Dongle over a radio link.

Install the `dongle-flash` tool by running the following command from the `tools/dongle-flash` directory.

``` console
$ cargo install --path . -f
```

The Dongle does not contain an on-board debugger, like the DK, so we cannot use `probe-rs` tools to write programs into it. Instead, the Dongle's stock firmware comes with a *bootloader*.

When put in bootloader mode the Dongle will run a bootloader program instead of the last application that was flashed into it. This bootloader program will make the Dongle show up as a USB CDC ACM device (AKA Serial over USB device) that accepts new application images over this interface. We'll use the `nrfutil` tool to communicate with the bootloader-mode Dongle and flash new images into it.

To put the Dongle in bootloader mode connect it to your laptop / PC  / mac and then press its *reset* button. The Dongle has two buttons: a round-ish user button (SW1) and a square-ish reset button (RESET); the latter is mounted "sideways". The buttons are next to each other. The RESET button is mounted closer to the edge of the board that has the Nordic logo on silkscreen and the actual button is facing towards that edge. The opposite edge of the board is narrower and has the surface USB connector; this is the end that goes into your PC USB port.

When the Dongle is in bootloader mode its red LED will oscillate in intensity. The Dongle will also appear as a USB CDC ACM device with vendor ID `0x1915` and product ID `0x521f`. If you have the `lsusb` tool installed (Linux distributions have it) then you can run it to check the presence of the USB device:

``` console
$ lsusb
(..)
Bus 001 Device 016: ID 1915:521f Nordic Semiconductor ASA 4-Port USB 2.0 Hub
```

If you don't have the `lsusb` tool installed, we have provided a cross-platform version of it in the `advanced/host/lsusb` folder. Call `cargo run` from that directory to run the tool.

``` console
$ cargo run
(..)
Bus 001 Device 016: ID 1915:521f <- nRF52840 Dongle (in bootloader mode)
```

Now that the device is in bootloader mode browse to the `boards/dongle` directory. You'll find some `*.hex` files there. These are pre-compiled Rust programs that have been converted into the Intel Hex format that the `nrfutil` tool expects.

For the next section you'll need to flash the `loopback.hex` file into the Dongle. There are two ways to do this. You can make 2 long `nrfutil` invocations or you can use our `dongle-flash` tool, which will invoke `nrfutil` for you. The `dongle-flash` way is shown below:

``` console
$ dongle-flash loopback.hex
packaging iHex using nrfutil ...
DONE
  [####################################]  100%
Device programmed.
```

After the device has been programmed it will automatically reset and start running the new application.

The `loopback` application will *blink* the red LED in a heartbeat fashion: two fast blinks (LED on then off) followed by two periods of silence (LED off). The application will also make the Dongle enumerate itself as a CDC ACM device. If you run our `lsusb` tool (from the `advanced/host/lsusb` directory) you should see the newly enumerated Dongle in the output:

``` console
$ cargo run
Bus 001 Device 020: ID 2020:0309 <- nRF52840 Dongle (loopback.hex)
```

The `loopback` app will log messages over the USB interface. To display these messages on the host we have provided a cross-platform tool: `serial-term`. Install it by running the following command from the `tools/serial-term` directory.

``` console
$ cargo install --path . -f
```

If you run the `serial-term` application you should see the following output:

``` console
$ serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm
(..)
```

> NOTE The application may take a while to print to the console

This line is printed by the `loopback` app on boot. It contains the device ID of the dongle, a 64-bit unique identifier (so everyone will see a different number); the radio channel that the device will use to communicate; and the transmission power of the radio in dBm.

Leave the Dongle connected and the `serial-term` application running. Now we'll switch back to the Development Kit.

## Radio out

Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-send.rs` file.

In this section you'll send radio packets from the DK to the Dongle and get familiar with the different settings of the radio API.

First run the program as it is. You should new output in the output of the `serial-term` program.

``` console
$ serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm
received 5 bytes (LQI=49)
```

The program broadcasts a radio packet that contains the 5-byte string `Hello` over channel 20 (which has a center frequency of 2450 MHz). The `loopback` program running on the Dongle is listening to all packets sent over channel 20; every time it receives a new packet it reports its length and the Link Quality Indicator (LQI) metric of the transmission over the USB/serial interface. As the name implies the LQI metric indicates how good the connection between the sender and the device is.

Now run the `radio-send` program several times with different variations:

- change the distance between the Dongle and the DK -- move the DK closer to or further away from the Dongle.
- change the transmit power
- change the channel
- change the length of the packet
- different combinations of all of the above

> NOTE if you decide to send many packets in a single program then you should use the `Timer` API to insert a delay of at least five milliseconds between the transmissions. This is required because the Dongle will use the radio medium right after it receives a packet. Not including the delay will result in the Dongle missing packets

The radio interface we are using follows the IEEE 802.15.4 specification but it's missing MAC level features like addressing (each device gets its own address), opt-in acknowledgment (a transmitted packet must be acknowledged with a response acknowledgment packet; the packet is re-transmitted if the packet is not acknowledged in time). These MAC level features are not implemented *in hardware* (in the nRF52840 Radio peripheral) so they would need to be implemented in software to be fully IEEE 802.15.4 compliant.

802.15.4 radios are often used in mesh networks like Wireless Sensors Networks (WSN). The devices, or *nodes*, in these networks can be mobile so the distance between nodes can change in time. To prevent a link between two nodes getting broken due to mobility the LQI metric is used to decide the transmission power -- if the metric degrades power should be increased, etc. At the same time, the nodes in these networks often need to be power efficient (e.g. are battery powered) so the transmission power is often set as low as possible -- again the LQI metric is used to pick an adequate transmission power.

## Radio in

In this section we'll explore the `recv` method of the Radio API. As the name implies, this is used to listen for packets. The method will block the program execution until a packet is received. We'll continue to use the Dongle in this section; it should be running the `loopback` application; and the `serial-term` application should also be running in the background.

The `loopback` application running on the Dongle will broadcast a radio packet after receiving one over channel 20. The contents of this outgoing packet will be the contents of the received one but reversed.

Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-recv.rs` file and click the "Run" button.

The Dongle will response as soon as it receives a packet. If you insert a delay between the `send` operation and the `recv` operation in the `radio-recv` program this will result in the DK not seeing the Dongle's response.

In a fully IEEE 802.15.4 compliant implementation one can mark a packet as "requires acknowledgment". The recipient must respond to these packets with an acknowledgment packet; if the sender doesn't receive the acknowledgment packet it will re-transmit the packet. This feature is part of the MAC layer and not implemented in the `Radio` API we are using so packet loss is possible even when the radios are close enough to communicate.

## Radio puzzle

> TODO(japaric) before this section maybe cover collision avoidance

For this section you'll need to flash the `puzzle.hex` program on the Dongle. Follow the instructions from the "nRF52840 Dongle" section but flash the `puzzle.hex` program instead of the `loopback.hex` one -- don't forget to put the Dongle in bootloader mode before invoking `dongle-flash`.

Like in the previous sections the Dongle will listen for radio packets over channel 20 while also logging messages over a USB/serial interface.

Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-puzzle.rs` file.

Your task in this section is to decrypt the [substitution cipher] encrypted *ascii* string stored in the Dongle. The string has been encrypted using *simple substitution*.

[substitution cipher]: https://en.wikipedia.org/wiki/Substitution_cipher

The Dongle will respond differently depending on the length of the incoming packet:

- On zero-sized packets it will respond with the encrypted string.
- On one-byte sized packets it will respond with the *direct* mapping from a *plaintext* letter -- the letter contained in the packet -- to the *ciphertext* letter.
- On packets of any other length the Dongle will respond with the string `correct` if it received the decrypted string, otherwise it will respond with the `incorrect` string.

Our suggestion is to use a dictionary / map. `std::collections::HashMap` is not available in `no_std` code (without linking to a global allocator) but you can use one of the maps in the [`heapless`] crate. To make this crate available in your application get the latest version from [crates.io] and add it to the `beginner/apps/Cargo.toml` file, for example:

[`heapless`]: https://docs.rs/heapless
[crates.io]: https://crates.io/crates/heapless

``` toml
# Cargo.toml
[dependencies]
heapless = "0.5.0"
```

If you haven't use a stack-allocated collection before note that you'll need to specify the capacity of the collection as a type parameter using one of the "type-level values" in the `heapless::consts` module. The crate level documentation of the `heapless` crate has some examples.

P.S. The plaintext string is *not* stored in `puzzle.hex` so running `strings` on it will not give you the answer.

---

> NOTE additional content, if needed / desired

## Starting a project from scratch

> cortex-m-quickstart

## (extra) adding addresses to packets

> have people use the `ieee802154` crate to add a MAC header to the radio packet. New dongle firmware would be required to respond differently to broadcast packets and addressed packets
