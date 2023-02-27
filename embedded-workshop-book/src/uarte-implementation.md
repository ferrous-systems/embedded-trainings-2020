# Write the Uarte implementation
## Step-by-Step Solution

### Check Documentation.

The UART protocol requires four pins, they are usually labelled:
* RXD
* TXD
* CTS
* RTS
  
Check the documentation to find out which pins are reserved for these and what their configuration needs to be.  

### Explore the `nrf-hal` to find out what needs to be done. 


The `nrf52840-hal` is a crate that exports all the `52840` flagged features from the `nrf-hal-common`. Let's take a look at the [Uarte module](https://github.com/nrf-rs/nrf-hal/blob/v0.14.1/nrf-hal-common/src/uarte.rs). 

In line 16 we see, that the nRF52840 uses the `hal::pac::UARTE1` peripheral.
In line 44 you find the `struct Uarte<T>(T)`, the interface to a UARTE instance `T`. Besides the instance `T`, the instantiating method takes variables of the following types as arguments: `Pins`, `Parity` and `Baudrate`.

A quick search of the document reveals where to find all of them:
* `Pins`: Line 463
* `Parity` and `Baudrate`: Re-export on line 34

Add the following lines as import:
```rust
use hal::pac::uarte0::{
    baudrate::BAUDRATE_A as Baudrate, config::PARITY_A as Parity};
use hal::uarte;
```

###  Add `struct Uarte` that serves as a wrapper for the `UARTE1` instance.

The struct has one field labelled `inner`, it contains the `UARTE1` instance.

### Bring up the peripheral in the `fn init()`

Take a closer look at the definition of the `Pins` struct. Import the types of the pin configuration that you don't have yet. Note that the third and fourth pin are each wrapped in an `Option`. 
Level?
Create an instance of this struct in `fn init()` with the appropriate pins and configurations.

Create an interface to the UARTE1 instance with `uarte::Uarte::new(...)`. The UARTE0 instance can be found in the `periph` variable. Set parity to `INCLUDED` and the baud rate to `BAUD115200`.


### Board struct

Add a field for the `Uarte` struct in the Board struct. 
add the field to the instance of the Board struct in `fn init()`.

### Implementing the `fmt::Write` trait

We can't just write to the Uarte instance. A simple write would write from flash memory. This does not work because of EasyDMA. We have to write a function that implements the `fmt::Write` trait. This trait guarantees that the buffer is fully and successfully written on a stack allocated buffer, before it returns. 

What exactly does the trait guarantee?

Create a public method `write_str`. It takes a mutable reference to self and a `&str` as argument. It returns an `fmt::Result`

Create a buffer. The type is an array of 16 u8, set to all 0. 

To copy all data into an on-stack buffer, iterate over every chunk of the string to copy it into the buffer:

```rust
for block in string.as_bytes().chunks(16) {
    buf[..block.len()].copy_from_slice(block);
    self.inner.write(&buf[..block.len()]).map_err(|_| fmt::Error)?;
}
```
return `Ok(())`

### Connect your computer to the virtual UART
   
Use the following command to find the address of the nRF52840-DK on your computer. 

```
ls /dev/tty*
```

Run the following command to run `screen` with the nRF52840-DK with 115200 baud. 

```
screen <adress of mc> 115200
```

### Run the example.
   
In another terminal window go into the folder `down-the-stack/apps`.

Use the following command. 
```
cargo run --bin uarte_print
```

On your terminal window where `screen` runs, "Hello, World" should appear. 