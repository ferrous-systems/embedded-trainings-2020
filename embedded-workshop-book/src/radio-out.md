# Radio Out

✅ Open the `src/bin/radio-send.rs` file.

In this section you'll send radio packets from the DK to the Dongle and get familiar with the different settings of the radio API.

✅ First run the program `radio-send.rs` as it is. You should see new output in the output of the `serial-term` program.

``` console
$ serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm app=loopback.hex
received 5 bytes (LQI=49)
```

The program broadcasts a radio packet that contains the 5-byte string `Hello` over channel 20 (which has a center frequency of 2450 MHz). The `loopback` program running on the Dongle is listening to all packets sent over channel 20; every time it receives a new packet it reports its length and the Link Quality Indicator (LQI) metric of the transmission over the USB/serial interface. As the name implies the LQI metric indicates how good the connection between the sender and the receiver is.

## Slices

The `send` method takes a *reference* -- in Rust, a reference (`&`) is a non-null pointer that's compile-time known to point into valid (e.g. non-freed) memory --  to a `Packet` as argument. A `Packet` is a stack-allocated, fixed-size buffer. You can fill the `Packet` (buffer) with data using the `copy_from_slice` method -- this will overwrite previously stored data.

This `copy_from_slice` method takes a *slice of bytes* (`&[u8]`). A slice is a reference into a list of elements stored in contiguous memory. One way to create a slice is to take a reference to an *array*, a fixed-size list of elements stored in contiguous memory.

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

// now point to the other array
slice = &array2;
log::info!("{:?}", slice); // length = 4
```

## Byte literals

In the example we sent the list of bytes: `[72, 101, 108, 108, 111]`, which can be interpreted as the string `"Hello"`. To see why this is the case check this [list of printable ASCII characters][ascii]. You'll see that letter `H` is represented by the (single-byte) value `72`, `e` by `101`, etc.

[ascii]: https://en.wikipedia.org/wiki/ASCII#Printable_characters

Rust provides a more convenient way to write ASCII characters: byte literals. `b'H'` is syntactic sugar for the literal `72u8`, `b'e'` is equivalent to `101u8`, etc.. So we can rewrite `[72, 101, 108, 108, 111]` as `[b'H', b'e', b'l', b'l', b'o']`. Note that byte literals can also represent `u8` values that are not printable ASCII characters: those values are written using escaped sequences like `b'\x7F'`, which is equivalent to `0x7F`.

## Byte string literals

`[b'H', b'e', b'l', b'l', b'o']` can be further rewritten as `b"Hello"`. This is called a *byte* string literal (note that unlike a string literal like `"Hello"` this one has a `b` before the opening double quote). A byte string literal is a series of byte literals (`u8` values); these literals have type `&[u8; N]` where `N` is the number of byte literals in the string.

Because byte string literals are references you need to *dereference* them to get an array type.

``` rust
let reftoarray: &[u8; 2] = b"Hi";

// these two are equivalent
let array1:  [u8; 2] = [b'H', 'i'];
let array2:  [u8; 2] = *b"Hi";
//          ^          ^ dereference
```

Or if you want to go the other way around: you need to take a reference to an array to get the same type as a byte string literal.

``` rust
// these two are equivalent
let reftoarray1: &[u8; 2] = b"Hi";
let reftoarray2: &[u8; 2] = &[b'H', 'i'];
//               ^          ^
```
## Character constraints in byte string vs. string literals

You can encode text as `b"Hello"` or as `"Hello"`.

`b"Hello"` is by definition a string (series) of byte literals so each character has to be a byte literal like `b'A'` or `b'\x7f'`. You cannot use "Unicode characters" (`char` type) like emoji or CJK (Chinese Japanese Korean) in byte string literals.

On the other hand, `"Hello"` is a string literal with type `&str`. `str` strings in Rust contain UTF-8 data so these string literals can contain CJK characters, emoji, Greek letters, Cyrillic script, etc.

## Printing strings and characters

In this workshop we'll work with ASCII strings so byte string literals that contain no escaped characters are OK to use as packet payloads.

You'll note that `log::info!("{:?}", b"Hello")` will print `[72, 101, 108, 108, 111]` rather than `"Hello"` and that the `{}` format specifier (`Display`) does not work. This is because the type of the literal is `&[u8; N]` and in Rust this type means "bytes"; those bytes could be ASCII data, UTF-8 data or something else.

To print this you'll need to convert the slice `&[u8]` into a string (`&str`) using the `str::from_utf8` function. This function will verify that the slice contains well formed UTF-8 data and interpret it as a UTF-8 string (`&str`). As long as we use ASCII data (printable ASCII characters) this conversion will not fail.

Something similar will happen with byte literals: `log::info!("{}", b'A')` will print `65` rather than `A`. To get the `A` output you can cast the byte literal (`u8` value) to the `char` type: `log::info!("{}", b'A' as char)`.

## Link Quality Indicator (LQI)

Now run the `radio-send` program several times with different variations:

- change the distance between the Dongle and the DK -- move the DK closer to or further away from the Dongle.
- change the transmit power
- change the channel
- change the length of the packet
- different combinations of all of the above

Take note of how LQI changes with these changes. Does packet loss occur in any of these configurations?

> NOTE if you decide to send many packets in a single program then you should use the `Timer` API to insert a delay of at least five milliseconds between the transmissions. This is required because the Dongle will use the radio medium right after it receives a packet. Not including the delay will result in the Dongle missing packets

802.15.4 radios are often used in mesh networks like Wireless Sensors Networks (WSN). The devices, or *nodes*, in these networks can be mobile so the distance between nodes can change in time. To prevent a link between two nodes getting broken due to mobility the LQI metric is used to decide the transmission power -- if the metric degrades power should be increased, etc. At the same time, the nodes in these networks often need to be power efficient (e.g. are battery powered) so the transmission power is often set as low as possible -- again the LQI metric is used to pick an adequate transmission power.

## 802.15.4 compatibility

The radio API we are using follows the PHY layer of the IEEE 802.15.4 specification, but it's missing MAC level features like addressing (each device gets its own address), opt-in acknowledgment (a transmitted packet must be acknowledged with a response acknowledgment packet; the packet is re-transmitted if the packet is not acknowledged in time). These MAC level features are not implemented *in hardware* (in the nRF52840 Radio peripheral) so they would need to be implemented in software to be fully IEEE 802.15.4 compliant.

This is not an issue for the workshop exercises but it's something to consider if you would like to continue from here and build a 802.15.4 compliant network API.
