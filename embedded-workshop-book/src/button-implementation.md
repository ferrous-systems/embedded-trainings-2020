# Write the Button Implementation
## Step-by-Step Solution

### Step 1: Read the docs! 

✅ Go to [Nordic Infocenter](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html) to download the **User Guide**. 

✅ Read docs, section 8.7 for info about pins and pin configuration related to the buttons. Note down the pins that the buttons are connected to. 

The pins need to be configured as input pins with an internal pull-up. The pins as well as the configurations are defined as types in the `nrf-hal` in the `gpio` peripheral. Add the following imports: `Input` and `PullUp`.

### Step 2: Add the structs that represent the buttons as a group and a generic single button. 
   
✅ Add the struct that represents the single button. It has only one field, `inner`. The type of this button is the pin configuration: `Pin<Input<PullUp>>`

✅ Add the `struct` that represents the group of buttons has four fields, one for each button. The field name contains the number that corresponds to the button numeration on the board. The of type of each field is the struct that represents the generic single button. 

✅ Add doc comments for every struct and field!

Building this code should return a warning: field `inner` is never read.

<details>
  <summary>Solution</summary>

```rust
/// All buttons on the board
pub struct Buttons {
    /// BUTTON1: pin P0.11, green LED
    pub b_1: Button,
    /// BUTTON2: pin P0.12, green LED
    pub b_2: Button,
    /// BUTTON3: pin P0.24, green LED
    pub b_3: Button,
    /// BUTTON4: pin P0.25, green LED
    pub b_4: Button,
}

/// A single button
pub struct Button {
    inner: Pin<Input<PullUp>>,
}
```
</details>

### Step 3: Implement the button function. 

✅ Add an `impl` block for the `struct Button`. Add a method `is_pushed` that takes in the struct as `&self` and returns a bool, if the button is pushed. 

✅ Now remember, the pins the buttons are connected to are configured as active low. For buttons this means, that the pin is pulled low, when the button is pushed. 

✅ In the `nrf-hal` you can find a method to check if a single pin is low. To use it, you have to add the following line to your `nrf52840_hal` imports: `prelude::InputPin`.


<details>
  <summary>Solution</summary>

```rust
impl Button {
    /// returns true if button is pushed 
    pub fn is_pushed(&self) -> bool {
        self.inner.is_low() == Ok(true)
    }
}
```
</details>

### Step 4: Bring up the pins!

✅ Go to `pub fn init()`, the function that initializes the board's peripherals. 

✅ Configure each pin as degraded, pull-up input pin and bind it to a variable that makes it clear what button number it is connected to. 

Building this code brings up warnings about unused variables. 

<details>
  <summary>Solution</summary>

```rust
// Buttons
let b_1 = pins.p0_11.degrade().into_pullup_input();
let b_2 = pins.p0_12.degrade().into_pullup_input();
let b_3 = pins.p0_24.degrade().into_pullup_input();
let b_4 = pins.p0_25.degrade().into_pullup_input();
```
</details>

### Step 5: Add everything to the board struct. 

✅ In the definition of the `struct Board` add a field for the `struct Buttons`.

✅ In the pub `fn init()` function, where `Board` is instantiated, add the button field, assigning the pins you defined earlier to the respective buttons. 

<details>
  <summary>Solution</summary>

```rust
/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Buttons
    pub buttons: Buttons,
    /// Timer
    pub timer: Timer,
}

// ...

pub fn init() -> Result<Board, ()> {
    // ...
    Ok(Board {
            leds: Leds {
                // ...
            },
            buttons: Buttons {
                b_1: Button { inner: b_1},
                b_2: Button { inner: b_2},
                b_3: Button { inner: b_3},
                b_4: Button { inner: b_4},
            },
            timer: Timer { inner: timer },
        })
    } else {
        Err(())
    }

}

```
</details>

### Step 6: Run the example!

✅ Go to `/down-the-stack/apps`.

✅ Run the following command:

```shell
cargo run --bin button
```

### Step 7: Generate the docs!

✅ Out of the apps folder run the following command to build the docs for this crate and to view your written documentation!

```shell
cargo doc
```