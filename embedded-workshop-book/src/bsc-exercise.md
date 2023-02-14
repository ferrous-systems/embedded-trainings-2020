# BSC Exercise

In this exercise you will learn how to write a board support crate. 
The bsc template will already contain the led and timer implementation. 
The radio and USB/Power implementations will be deleted, because that just takes up unnecessary space and adds to confusion. 


## Learning goals
* implement buttons functionality 
* uarte implementation
* impl blocks, associated functions, methods
* generate docs!


## Steps
### Write a button implementation
* add field in the board struct
* add struct for all buttons
* add struct for the single button

* Read docs, section 8.7 for info about pins and pin configuration
* add button bring up to board init

* add doc lines every where!
* add methods in impl block:
  * detect button push
  * debounce button function? like in knurling session, requires implementation of a second timer, just for this?

### Write Uarte implementation
* add field to the board struct
* add struct for the instance, how to figure out what the type of the inner field is
* create instance in init, add baudrate, parity etc. 
* add to instantiation of board struct
* impl fmt::Write for the Uarte struct, simple write does not work because of dma 
* example code with button is not a good idea for the simple button implementation.

I think this is plenty for an hour. 