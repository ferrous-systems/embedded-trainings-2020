# Write the Uarte implementation
## Step-by-Step Solution

### Step 1: Check Documentation.

The UART protocol requires four pins, they are usually labelled:
* RXD
* TXD
* CTS
* RTS

✅ Check the User Guide in section 7.2 to find to find out which pins are reserved for these and what their configuration needs to be.  

### Step 2: Explore the `nrf-hal` to find out what needs to be done. 


The `nrf52840-hal` is a crate that exports all the `52840` flagged features from the `nrf-hal-common`. Let's take a look at the nRF-Hal [Uarte module](https://github.com/nrf-rs/nrf-hal/blob/v0.14.1/nrf-hal-common/src/uarte.rs). 

In line 16 we see, that the nRF52840 uses the `hal::pac::UARTE1` peripheral.
In line 44 you find the `struct Uarte<T>(T)`, the interface to a UARTE instance `T`. Besides the instance `T`, the instantiating method takes variables of the following types as arguments: `Pins`, `Parity` and `Baudrate`.

A quick search of the document reveals where to find all of them:
* `Pins`: Line 463
* `Parity` and `Baudrate`: Re-export on line 34

✅ Add the following lines as import:
```
use hal::pac::uarte0::{
    baudrate::BAUDRATE_A as Baudrate, config::PARITY_A as Parity};
use hal::uarte;
```

###  Step 3: Add `struct Uarte`

✅ Add `struct Uarte` that serves as a wrapper for the `UARTE1` instance.
The struct has one field labelled `inner`, it contains the `UARTE1` instance: `hal::Uarte<hal::pac::UARTE1>`.

<details>
  <summary>Solution</summary>

```rust
pub struct Uarte {
    inner: hal::Uarte<hal::pac::UARTE1>,
}
```
</details>

### Step 4: Bring up the peripheral in the `fn init()`

✅ Take a closer look at the definition of the `uarte::Pins` struct in the `nrf-hal`. Compare the pin type configurations with the ones you have already imported in `lib.rs`. Add the ones you're missing. 

✅ Create an instance of this struct in `fn init()` with the appropriate pins and configurations. Set the output pin's level to `Level::High`.
Note, that the third and fourth pin are each wrapped in an `Option`. 

✅ Create an interface to the UARTE1 instance with `uarte::Uarte::new(...)` that you bind to a variable. This instantiating method takes four arguments:
* The `UARTE1` instance can be found in the `periph` variable.
* Your instance of the `uarte::Pins` struct.
* Set parity to `Parity::INCLUDED` 
* set the baud rate to `Baudrate::BAUD115200`.

<details>
  <summary>Solution</summary>

```rust
  let pins =  hal::uarte::Pins {
            rxd: pins.p0_08.degrade().into_floating_input(),
            txd: pins.p0_06.degrade().into_push_pull_output(Level::High),
            cts: Some(pins.p0_07.degrade().into_floating_input()),
            rts: Some(pins.p0_05.degrade().into_push_pull_output(Level::High)),
        };
       

        let uarte = hal::uarte::Uarte::new(periph.UARTE1, pins, Parity::INCLUDED, Baudrate::BAUD115200);
```
</details>

### Step 5: Board struct

✅  Add a field for the `Uarte` struct in the `Board` struct. 
add the field to the instance of the `Board` struct in `fn init()`.

<details>
  <summary>Solution</summary>

```rust

pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Buttons
    pub buttons: Buttons,
    /// Timer
    pub timer: Timer,
    /// uarte interface
    pub uarte: Uarte,
}

// ...

pub fn init() -> Result<Board, ()> {

    // ... 

        Ok(Board {
            leds: Leds {
                // ...
            },

            buttons: Buttons {
                // ...
            },
            // 🔼  --- Button Exercise --- 🔼 

            timer: Timer { inner: timer },

            uarte: Uarte { inner: uarte },
        })
    } else {
        Err(())
    }
```

</details>

### Step 6: Implementing the `fmt::Write` trait

We want to implement the `fmt::Write` trait so that users can call `write!` on our Uarte object

When implementing this, we can't just write to the `Uarte` instance because a simple write of a string literal would try and read the string literal from flash memory. This does not work because the EasyDMA peripheral in the nRF52 series can only access RAM, not flash. 

Instead our implementation must ensure all the strings are copied to a stack allocated buffer and that buffer is passed to the Uarte's `write` method. 

✅ Add `use::core::fmt;` to your imports.

✅ Create a public method `write_str`. It takes a mutable reference to self and a `&str` as argument. It returns an `fmt::Result`

✅ Create a buffer. The type is an `array` of 16 u8, set to all 0. 

✅ To copy all data into an on-stack buffer, iterate over every chunk of the string to copy it into the buffer.

<details>
  <summary>Solution</summary>

```rust
impl fmt::Write for Uarte {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Copy all data into an on-stack buffer so we never try to EasyDMA from
        // flash.
        let mut buf: [u8; 16] = [0; 16];
        for block in s.as_bytes().chunks(16) {
            buf[..block.len()].copy_from_slice(block);
            self.inner.write(&buf[..block.len()]).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}
```
</details>

### Step 7: Connect your computer to the virtual UART
[todo!] [directions for mac present, linux and windows are missing.]
   
✅  Use the following command to find the address of the nRF52840-DK on your computer. 

```
ls /dev/tty.usbmodem*
```

✅  Run the following command to run `screen` with the nRF52840-DK with 115200 baud. 

```
screen <address of mc> 115200
```

### Step 8: Run the example.
   
✅ In another terminal window go into the folder `down-the-stack/apps` and use the following command. 

```
cargo run --bin uarte_print
```

On your terminal window where `screen` runs, "Hello, World" should appear. 

You need to terminate `screen` manually.