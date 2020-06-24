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

Open the `beginner/apps` folder in VS Code.

``` console
$ # or use "File > Open Folder" in VS Code
$ code beginner/apps
```

Then open the `src/bin/hello.rs` file.

If you find it more convenient to open the repository at the root then please also add the `beginner/apps` folder to the VS Code workspace: right click the left side panel, select "Add folder to workspace" and add the `beginner/apps` folder.

Note the differences between this embedded program and a desktop program:

 The `#![no_std]` attribute indicates that the program will not make use of the standard library, `std` crate. Instead it will use the `core` library, a subset of the standard library that does not on a underlying operating system (OS).

The `#![no_main]` attribute indicates that the program will use a custom entry point instead of the default `fn main() { .. }` one.

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

> NOTE if you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear

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

1. Run this command from the `tools/dk-run` folder:

``` console
$ cargo install --path . -f
```

2. Open the `src/bin/hello.rs` file and click the "Run" button that's hovering over the `main` function.

Note: you will get the "Run" button if the Rust analyzer's workspace is set to the `beginner/apps` folder. This will be the case if the current folder in VS code (left side panel) is set to `beginner/apps`.

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

Open the `src/bin/panic.rs` file and click the "Run" button.

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

Open the `src/bin/led.rs` file.

The `dk` crate / library is a Hardware Abstraction Layer (HAL) over the nRF52840 Development Kit. The purpose of a HAL is to abstract away the device-specific details of the hardware, for example registers, and instead expose a higher level API more suitable for application development.

The `dk::init` function we have been calling in all programs initializes a few of the nRF52840 peripherals and returns a `Board` structure that provides access to those peripherals. We'll first look at the `Leds` API. Open the documentation for the `dk` crate running the following command from the `beginner/apps` folder:

``` console
$ cargo doc -p dk --open
```

Check the API docs of the `Led` abstraction then run the `led` program. Two of the green LEDs on the board should turn on; the other two should stay off.

> NOTE this program will not terminate itself. Within VS code you need to click "Kill terminal" (garbage bin icon) in the bottom panel to terminate it.

Now, uncomment the `log::set_max_level` line. This will make the logs more verbose; they will now include logs from the board initialization function (`dk::init`) and from the `Led` API.

Among the logs you'll find the line "I/O pins have been configured for digital output". At this point the electrical pins of the nRF52840 microcontroller has been configured to drive the 4 LEDs on the board.

After the `dk::init` logs you'll find logs about the `Led` API. As the logs indicate an LED becomes active when the output of the pin is a *logical zero*, which is also referred as the "low" state. This "active low" configuration does not apply to all boards: it depends on how the pins have been wired to the LEDs. You should refer to the [board documentation] to find out which pins are connected to LEDs and whether "active low" or "active high" applies to it.

[board documentation]: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fintro.html

## Timers and time

Next we'll look into the time related APIs exposed by the `dk` HAL.

Open the `src/bin/blinky.rs` file.

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

When the Dongle is in bootloader mode its red LED will oscillate in intensity. The Dongle will also appear as a USB CDC ACM device with vendor ID `0x1915` and product ID `0x521f`.

In the `tools` folder you'll find `usb-list`: a minimal cross-platform version of the `lsusb` tool. Run it (`cargo run` from `tools/usb-list`) to list all USB devices; the Dongle will be highlighted in the output.

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

The `loopback` application will *blink* the red LED in a heartbeat fashion: two fast blinks (LED on then off) followed by two periods of silence (LED off). The application will also make the Dongle enumerate itself as a CDC ACM device. If you run  `usb-list` tool (from the `tools/usb-list` directory) you should see the newly enumerated Dongle in the output:

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

This line is printed by the `loopback` app on boot. It contains the device ID of the dongle, a 64-bit unique identifier (so everyone will see a different number); the radio channel that the device will use to communicate; and the transmission power of the radio in dBm.

Leave the Dongle connected and the `serial-term` application running. Now we'll switch back to the Development Kit.

## Radio out

Open the `src/bin/radio-send.rs` file.

In this section you'll send radio packets from the DK to the Dongle and get familiar with the different settings of the radio API.

First run the program as it is. You should new output in the output of the `serial-term` program.

``` console
$ serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm
received 5 bytes (LQI=49)
```

The program broadcasts a radio packet that contains the 5-byte string `Hello` over channel 20 (which has a center frequency of 2450 MHz). The `loopback` program running on the Dongle is listening to all packets sent over channel 20; every time it receives a new packet it reports its length and the Link Quality Indicator (LQI) metric of the transmission over the USB/serial interface. As the name implies the LQI metric indicates how good the connection between the sender and the receiver is.

### Slices

The `send` method takes a *slice of bytes* (`&[u8]`). A slice is a *reference* -- in Rust, a reference (`&`) is a non-null pointer that's compile-time known to point into valid (e.g. non-freed) memory -- into a list of elements stored in contiguous memory. One way to create a slice is to take a reference to an *array*, a fixed-size list of elements stored in contiguous memory.

``` rust
// stack allocated array
let array: [u8; 3] = [0, 1, 2];

let ref_to_array: &[u8; 3] = &array;
let slice: &[u8] = &array;
```

`slice` and `ref_to_array` are constructed in the same way but have different types. `ref_to_array` is represented in memory as a single pointer (1 word / 4 bytes); `slice` is represented as a pointer + length (2 words / 8 bytes).

Because slices track length at runtime rather than in their type they can point to chunks of memory of any length.

``` rust
let array1: [u8; 3] = [0, 1, 2];
let array2: [u8; 4] = [0, 1, 2, 3];

let mut slice: &[u8] = &array1;
log::info!("{:?}", slice); // length = 3

slice = &array2;
log::info!("{:?}", slice); // length = 4
```

### Byte literals

In the example we sent the list of bytes: `[72, 101, 108, 108, 111]`, which can be interpreted as the string `"Hello"`. To see why this is the case check this [list of printable ASCII characters][ascii]. You'll see that letter `H` is represented by the (single-byte) value `72`, `e` by `101`, etc.

[ascii]: https://en.wikipedia.org/wiki/ASCII#Printable_characters

Rust provides a more convenient way to write ASCII characters: byte literals. `b'H'` is syntactic sugar for the literal `72u8`, `b'e'` is equivalent to `101u8`, etc.. So we can rewrite `[72, 101, 108, 108, 111]` as `[b'H', b'e', b'l', b'l', b'o']`. Note that byte literals can also represent `u8` values that are not printable ASCII characters: those values are written using escaped sequences like `b'\x7F'`, which is equivalent to `0x7F`.

### Byte string literals

`[b'H', b'e', b'l', b'l', b'o']` can be further rewritten as `b"Hello"`. This is called a byte string literal; note that there's a `b` before the opening double quote. A byte string literal is a series of byte literals; these literals have type `&[u8; N]` where `N` is the number of byte (literals) in the string.

### Character constrains in byte string vs. string literals

You can encode text as `b"Hello"` or as `"Hello"`.

`b"Hello"` is by definition a string (series) of byte literals so each character has to be a byte literal like `b'A'` or `b'\x7f'`. You cannot use "Unicode characters" (`char` type) like emoji or CJK (Chinese Japanese Korean) in byte string literals.

On the other hand, `"Hello"` is a string literal with type `&str`. `str` strings in Rust contain UTF-8 data so these string literals can contain CJK characters, emoji, Greek letters, Cyrillic script, etc.

### Printing strings and characters

In this workshop we'll work with ASCII strings so byte string literals that contain no escaped characters are OK to use as packet payloads.

You'll note that `log::info!("{:?}", b"Hello")` will print `[72, 101, 108, 108, 111]` rather than `"Hello"` and that the `{}` format specifier (`Display`) does not work. This is because the type of the literal is `&[u8; N]` and in Rust this type means "bytes"; those bytes could be ASCII data, UTF-8 data or something else.

To print this you'll need to convert the slice `&[u8]` into a string (`&str`) using the `str::from_utf8` function. This function will verify that the slice contains well formed UTF-8 data and interpret it as a UTF-8 string (`&str`). As long as we use ASCII data (printable ASCII characters) this conversion will not fail.

### Link Quality Indicator (LQI)

Now run the `radio-send` program several times with different variations:

- change the distance between the Dongle and the DK -- move the DK closer to or further away from the Dongle.
- change the transmit power
- change the channel
- change the length of the packet
- different combinations of all of the above

Take note of how LQI changes with these changes. Do packet loss occur in any of these configurations?

> NOTE if you decide to send many packets in a single program then you should use the `Timer` API to insert a delay of at least five milliseconds between the transmissions. This is required because the Dongle will use the radio medium right after it receives a packet. Not including the delay will result in the Dongle missing packets

802.15.4 radios are often used in mesh networks like Wireless Sensors Networks (WSN). The devices, or *nodes*, in these networks can be mobile so the distance between nodes can change in time. To prevent a link between two nodes getting broken due to mobility the LQI metric is used to decide the transmission power -- if the metric degrades power should be increased, etc. At the same time, the nodes in these networks often need to be power efficient (e.g. are battery powered) so the transmission power is often set as low as possible -- again the LQI metric is used to pick an adequate transmission power.

### 802.15.4 compatibility

The radio interface we are using follows the IEEE 802.15.4 specification but it's missing MAC level features like addressing (each device gets its own address), opt-in acknowledgment (a transmitted packet must be acknowledged with a response acknowledgment packet; the packet is re-transmitted if the packet is not acknowledged in time). These MAC level features are not implemented *in hardware* (in the nRF52840 Radio peripheral) so they would need to be implemented in software to be fully IEEE 802.15.4 compliant.

## Radio in

In this section we'll explore the `recv_timeout` method of the Radio API. As the name implies, this is used to listen for packets. The method will block the program execution until a packet is received or the specified timeout has expired. We'll continue to use the Dongle in this section; it should be running the `loopback` application; and the `serial-term` application should also be running in the background.

The `loopback` application running on the Dongle will broadcast a radio packet after receiving one over channel 20. The contents of this outgoing packet will be the contents of the received one but reversed.

Open the `src/bin/radio-recv.rs` file and click the "Run" button.

The Dongle expects the packet to contain only ASCII characters and will not respond to packets that contain non-ASCII data. If you only send packets that contain byte string literals *with no escaped characters* (e.g. `b"hello"`) then this requirement will be satisfied. At the same time the Dongle will always respond with ASCII data so calling `str::from_utf8` on the response should never fail, unless the packet contents got corrupted in the transmission but the CRC should detect this scenario.

The Dongle will respond as soon as it receives a packet. If you insert a delay between the `send` operation and the `recv` operation in the `radio-recv` program this will result in the DK not seeing the Dongle's response. So try this: add a `timer.delay(x)` call before the `recv_timeout` call; try different values of `x` and observe what happens.

Having log statements between `send` and `recv_timeout` can also cause packets to be missed so try to keep those two calls as close to each other as possible and with as little code in between as possible.

> NOTE Packet loss can always occur in wireless networks, even if the radios are close to each other. The `Radio` API we are using will not detect lost packets because it does not implement IEEE 802.15.4 Acknowledgement Requests. If you are having trouble with lost packets, consider adding a retry loop.

## Radio puzzle

For this section you'll need to flash the `puzzle.hex` program on the Dongle. Follow the instructions from the "nRF52840 Dongle" section but flash the `puzzle.hex` program instead of the `loopback.hex` one -- don't forget to put the Dongle in bootloader mode before invoking `dongle-flash`.

Like in the previous sections the Dongle will listen for radio packets -- this time over *channel 25* -- while also logging messages over a USB/serial interface.

Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-puzzle.rs` file.

Your task in this section is to decrypt the [substitution cipher] encrypted *ASCII* string stored in the Dongle. The string has been encrypted using *simple substitution*.

[substitution cipher]: https://en.wikipedia.org/wiki/Substitution_cipher

The Dongle will respond differently depending on the length of the incoming packet:

- On zero-sized packets it will respond with the encrypted string.
- On one-byte sized packets it will respond with the *direct* mapping from a *plaintext* letter (single `u8` value) -- the letter contained in the packet -- to the *ciphertext* letter (`u8` value).
- On packets of any other length the Dongle will respond with the string `correct` if it received the decrypted string, otherwise it will respond with the `incorrect` string.

The Dongle will always respond with packets that are valid UTF-8 so you can use the `str::from_utf8` on the response packets.

Our suggestion is to use a dictionary / map. `std::collections::HashMap` is not available in `no_std` code (without linking to a global allocator) but you can use one of the stack-allocated maps in the [`heapless`] crate. A `Vec`-like buffer may also come in handy; `heapless` provides a stack-allocated, fixed-capacity `Vec` type.

`heapless` is already declared as a dependency in the Cargo.toml of the project so you can directly import it into the application code using a `use` statement.

[`heapless`]: https://docs.rs/heapless
[crates.io]: https://crates.io/crates/heapless

``` toml
use heapless::IndexMap; // a dictionary / map
use heapless::Vec; // like `std::Vec` but stack-allocated
```

If you haven't use a stack-allocated collection before note that you'll need to specify the capacity of the collection as a type parameter using one of the "type-level values" in the `heapless::consts` module. The crate level documentation of the `heapless` crate has some examples.

Something you will likely run into while solving this exercise are *character* literals (`'c'`) and *byte* literals (`b'c'`). The former has type [`char`] and represent a single Unicode "scalar value". The latter has type `u8` (1-byte integer) and it's mainly a convenience for getting the value of ASCII characters, for instance `b'A'` is the same as the `65u8` literal.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html

*IMPORTANT* you do not need to use the `str` or `char` API to solve this problem, other than for printing purposes. Work directly with slices of bytes (`[u8]`) and bytes (`u8`); and only convert those to `str` or `char` when you are about to print them.

P.S. The plaintext string is *not* stored in `puzzle.hex` so running `strings` on it will not give you the answer.

These are our recommended steps to tackle the problem. Each step is demonstrated in a separate example:

1. Send a one letter packet (e.g. `A`) to the radio to get a feel for how the mapping works. Then do a few more letters. Check out example `radio-puzzle-1`

2. Get familiar with the dictionary API. Do some insertions and look ups. What happens if the dictionary gets full? See `radio-puzzle-2`

3. Next, get mappings from the radio and insert them into the dictionary. See `radio-puzzle-3`

4. You'll probably want a buffer to place the plaintext in. We suggest using `heapless::Vec` for this. See `radio-puzzle-4` (NB It is also possible to decrypt the packet in place)

5. Simulate decryption: fetch the encrypted string and "process" each of its bytes. See `radio-puzzle-5`

6. Now merge steps 3 and 5: build a dictionary, retrieve the secret string and do the reverse mapping to decrypt the message. See `radio-puzzle-6`

7. As a final step, send the decrypted string to the Dongle and check if it was correct or not. See `radio-puzzle-7`

## Starting a project from scratch

So far we have been using a pre-made Cargo project to work with the nRF52840 DK. In this section we'll see how to create a new embedded project for any microcontroller.

### Identify the microcontroller

The first step is to identify the microcontroller you'll be working with. The information about the microcontroller you'll need is:

1. Its processor architecture and sub-architecture.

This information should be in the device's data sheet or manual. In the case of the nRF52840, the processor is an ARM Cortex-M4 core. With this information you'll need to select a compatible *compilation target*. `rustup target list` will show all the supported compilation targets.

``` console
$ rustup target list
(..)
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
```

The compilation targets will usually be named using the following format: `$ARCHITECTURE-$VENDOR-$OS-$ABI`, where the `$VENDOR` field is sometimes omitted. Bare metal and `no_std` targets, like microcontrollers, will often use `none` for the `$OS` field. When the `$ABI` field ends in `hf` it indicates that the output ELF uses the *hardfloat* ABI

The `thumb` targets listed above are all the currently supported ARM Cortex-M targets. The table below shows the mapping between compilation targets and ARM Cortex-M processors.

| Compilation target          | Processor                          |
| --------------------------- | ---------------------------------- |
| `thumbv6m-none-eabi`        | ARM Cortex-M0, ARM Cortex-M0+      |
| `thumbv7m-none-eabi`        | ARM Cortex-M3                      |
| `thumbv7em-none-eabi`       | ARM Cortex-M4, ARM Cortex-M7       |
| `thumbv7em-none-eabihf`     | ARM Cortex-M4*F*, ARM Cortex-M7*F* |
| `thumbv8m.base-none-eabi`   | ARM Cortex-M23                     |
| `thumbv8m.main-none-eabi`   | ARM Cortex-M33, ARM Cortex-M35P    |
| `thumbv8m.main-none-eabihf` | ARM Cortex-M33F, ARM Cortex-M35PF  |


The ARM Cortex-M ISA is backwards compatible so for example you could compile a program using the `thumbv6m-none-eabi` target and run it on an ARM Cortex-M4 microcontroller. This will work but using the `thumbv7em-none-eabi` results in better performance (ARMv7-M instructions will be emitted by the compiler) so it should be preferred. The opposite, compiling for `thumbv7em-none-eabi` and running the resulting

2. Its memory layout.

In particular, you need to identify how much Flash and RAM memory the device has and at which address the memory is exposed. You'll find this information in the device's data sheet or reference manual.

In the case of the nRF52840, this information is in section 4.2 (Figure 2) of its Product Specification (see the References section at the bottom of this document). The nRF52840 has:

- 1 MB of Flash that spans the address range: `0x0000_0000` - `0x0010_0000`.
- 256 KB of RAM that spans the address range: `0x2000_0000` - `0x2004_0000`.

### The `cortex-m-quickstart` project template

With all this information you'll be able to build programs for the target device. The [`cortex-m-quickstart`] project template provides the most frictionless way to start a new project for the ARM Cortex-M architecture -- for other architectures check out other project templates by the [rust-embedded] organization.

[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart
[rust-embedded]: https://github.com/rust-embedded/

The recommended way to use the quickstart template is through the [`cargo-generate`] tool:

[`cargo-generate`]: https://crates.io/crates/cargo-generate

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

But it may be difficult to install the `cargo-generate` tool on Windows due to its `libgit2` (C library) dependency. Another option is to download a snapshot of the quickstart template from GitHub and then fill in the placeholders in `Cargo.toml` of the snapshot.

Once you have instantiated a project using the template you'll need to fill in the device-specific information you collected in the two previous steps:

1. Change the default compilation target in `.cargo/config`

``` toml
[build]
target = "thumbv7em-none-eabi"
```

For the nRF52840 you can choose either `thumbv7em-none-eabi` or `thumbv7em-none-eabihf`. If you are going to use the FPU then select the `hf` variant.

2. Enter the memory layout of the chip in `memory.x`

```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
```

3. `cargo build` now will cross compile programs for your target device.

If there's no template or signs of support for a particular architecture under the rust-embedded organization then you can follow the [embedonomicon] to bootstrap support for the new architecture by yourself.

[embedonomicon]:https://docs.rust-embedded.org/embedonomicon/

### Flashing the program

To flash the program on the target device you'll need to identify the on-board debugger, if the development board has one. Or choose an external debugger, if the development board exposes a JTAG or SWD interface via some connector.

If the hardware debugger is supported by the `probe-rs` project -- for example J-Link, ST-Link or CMSIS-DAP -- then you'll be able to use `probe-rs`-based tools like `cargo-flash` and `cargo-embed`. This is the case of the nRF52840 DK: it has an on-board J-Link probe.

If the debugger is not supported by `probe-rs` then you'll need to use [OpenOCD] or vendor provided software to flash programs on the board.

[OpenOCD]: http://openocd.org/

If the board does not expose a JTAG, SWD or similar interface then the microcontroller probably comes with a bootloader as part of its stock firmware. In that case you'll need to use `dfu-util` or a vendor specific tool like `nrfutil` to flash programs onto the chip. This is the case of the nRF52840 Dongle.

### Getting output

If you are using one of the probes supported by `probe-rs` then you can use the [`rtt-target`] library to get text output on `cargo-embed`. The logging functionality we used in the examples is implemented using the `rtt-target` crate.

[`rtt-target`]: https://crates.io/crates/rtt-target

If that's not the case or there's no debugger on board then you'll need to add a HAL before you can get text output from the board.

### Adding a Hardware Abstraction Layer (HAL)

Now you can hopefully run programs and get output from them. To use the hardware features of the device you'll need to add a HAL to your list of dependencies. [crates.io], [lib.rs] and [awesome embedded Rust] are good places to search for HALs.

[crates.io]: https://crates.io/search?q=hal
[lib.rs]: https://lib.rs/search?q=hal
[awesome embedded Rust]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates

After you find a HAL you'll want to get familiar with its API through its [API docs] and [examples]. HAL do not always expose the exact same API, specially when it comes to initialization and configuration of peripherals. However, most HAL will implement the [`embedded-hal`] traits. These traits allow inter-operation between the HAL and [*driver* crates][drivers]. These driver crates provide functionality to interface external devices like sensors, actuators and radios over interfaces like I2C and SPI.

[API docs]: https://docs.rs/nrf52840-hal/0.10.0/nrf52840_hal/
[examples]: https://github.com/nrf-rs/nrf-hal/tree/master/examples
[`embedded-hal`]: https://crates.io/crates/embedded-hal
[drivers]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates

If no HAL is available for your device then you'll need to build one yourself. This is usually done by first generating a Peripheral Access Crate (PAC) from a [System View Description][SVD] (SVD) file using the [`svd2rust`] tool. The PAC exposes a low level, but type safe, API to modify the registers on the device. Once you have a PAC you can use of the many HALs on crates.io as a reference; most of them are implemented on top of `svd2rust`-generated PACs.

[SVD]: http://www.keil.com/pack/doc/CMSIS/SVD/html/index.html
[`svd2rust`]: https://crates.io/crates/svd2rust

---

> NOTE additional content, if needed / desired

## (extra) adding addresses to packets

> have people use the `ieee802154` crate to add a MAC header to the radio packet. New dongle firmware would be required to respond differently to broadcast packets and addressed packets

## References

- [nRF52840 Product Specification 1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
