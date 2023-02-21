# BSC Exercise

In this exercise you will learn how to write a board support crate. 
The template will already contain the LED and timer implementation. 

Note: Introduction to the exercise is a guided tour through the template and running the hello example. 


## Learning goals
* implement buttons functionality 
* UARTE implementation
* `impl` blocks, associated functions, methods
* generate docs!

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





