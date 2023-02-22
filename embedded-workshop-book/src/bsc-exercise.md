# BSC Exercise

In this exercise you will learn how to write a board support crate. 
The template `dk_bsc/src/lib.rs` already contains the LED and Timer implementations. You will implement the buttons and the UARTE peripheral. 

Note: Introduction to the exercise is a guided tour through the template, and it's architecture. Make the participants aware of the placeholders for their implementations. run the hello example on the unmodified lib. 


## You will learn how to
* modify the `init()` function that brings up the board's peripherals
* how to configure pins 
* how to write a function that checks the state of a pin
* write methods for a `struct`
* UARTE implementation
* implement a Trait
* to document and generate docs for your own library!

## Prerequesits

* 
## Tasks
* Write a button implementation. This entails the following steps
  * `struct Buttons` with 4 fields, that represents each of the four buttons 
  * `struct Button` that is a wrapper for the pin that a single button is connected to
  * a method `is_pushed` that checks if a single button is pushed. 
  * initialize the pins in `fn init()`
  * add the `struct Button` to the definition and instantiation of `struct Board`.
  * Run `apps/buttons.rs` to test. 
* Write a UARTE implementation. 

## Knowledge

## Representation of Peripherals
The boards peripherals are represented as nested structs. The `struct Board` contains fields that represent single peripherals or groups of peripherals as structs, which in turn either contain a field of the single peripheral or ...

You have to add structs to represent the buttons and the UARTE peripheral to the board struct. 

## Comments


## impl blocks

## visibility of structs, fields and functions: the pub keyword


## Hardware documentation for pin configuration





