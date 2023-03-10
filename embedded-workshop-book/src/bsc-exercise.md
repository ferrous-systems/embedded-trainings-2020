# BSC Exercise

In this exercise you will learn how to write a board support crate by implementing buttons and the UARTE peripheral. 

The template `down-the-stack/dk_bsc/src/lib.rs` already contains the LED and Timer implementations. Add your code to the designated lines. You'll find a //todo! there. 

You can test after each step by running the following command out of `down-the-stack/apps`
```
cargo run --bin hello
```
This program will not call any of the functions you are implementing, so it does not matter if they are incomplete. It will refuse to build if there are errors present in the `lib.rs`!

`down-the-stack/dk_bsc/src/lib_solution.rs` contains the full solution code. 

Note for trainer: Introduction to the exercise is a guided tour through the template, and it's architecture. Make the participants aware of the placeholders for their implementations. run the hello example on the unmodified lib. 


## You will learn how to
* modify the `init()` function that brings up the board's peripherals
* how to configure pins 
* how to write a function that checks the state of a pin
* write methods for a `struct`
* UARTE implementation
* implement a Trait
* to document and generate docs for your own library!

## Prerequisites
[todo!]

## Tasks
* Write a button implementation. This entails the following steps:
  ✅ Add `struct Buttons` with 4 fields, that represents each of the four buttons.
  ✅ Add `struct Button` that is a wrapper for the pin that a single button is connected to.
  ✅ Write a method `is_pushed` that checks if a single button is pushed. 
  ✅ Initialize the pins in `fn init()`.
  ✅ Add the `struct Button` to the definition and instantiation of `struct Board`.
  ✅ Run `apps/buttons.rs` to test. 
  ✅ Run `cargo doc` out of the apps folder to find all your doc comments!
* Write a UARTE implementation. This entails the following steps:
  ✅ Check the `uarte` module of the `nrf-hal` for requirements of the instantiating method.
  ✅ Add `struct Uarte` that serves as wrapper for the `UARTE1` instance.
  ✅ Initialize the UARTE1 peripheral in `fn init()` using the following settings:
  * parity: included
  * baudrate: 115200 baud
  ✅ Add `struct Uarte` to the definition and instantiation of `struct Board`.
  ✅ Implement the `fmt::Write` trait for `struct Uarte`.
  ✅ Connect your computer to the virtual UART port with `screen`
  ✅ Run `apps/uarte_print.rs` to test.
## Knowledge

## Representation of Peripherals
The boards peripherals are represented as nested structs. The `struct Board` contains fields that represent single peripherals or groups of peripherals as structs, which in turn either contain a field of the single peripheral or ...

You have to add structs to represent the buttons and the UARTE peripheral to the board struct. 
[todo!]

## Comments
The `lib.rs` has an attribute #![deny(missing_docs)]. This means, that missing doc comments for structs are returned as compiler errors, to remind you to document your work properly. 

```rust
/// This is a doc comment
// This is a normal comment
```

## impl blocks
[todo!]
## Visibility of structs, fields and functions: the pub keyword
[todo!]

## Hardware documentation for pin configuration

Go to [Nordic Infocenter](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html) to download the User Guide. You can find all the information that is relevant to this exercise in there.



