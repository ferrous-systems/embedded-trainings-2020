# Write the Button Implementation
## Step-by-Step Solution

### Step 1: Read the docs! 
   
Go to [Nordic Infocenter](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html) to download the User Guide. Read docs, section 8.7 for info about pins and pin configuration related to the buttons. Note down the pins that the buttons are connected to. 
The pins need to be configured as input pins with an internal pull-up. The pins as well as the configurations are defined as types in the `nrf-hal` in the `gpio` peripheral. Add the following imports: `Input` and `PullUp`.

### Step 2: Add the structs that represent the buttons as a group and a generic single button. 
   
Add the struct that represents the single button. It has only one field, `inner`. The type of this button is the pin configuration: `Pin<Input<PullUp>>`

Add the `struct` that represents the group of buttons has four fields, one for each button. The field name contains the number that corresponds to the button numeration on the board. The of type of each field is the struct that represents the generic single button. 

Add doc comments for every struct and field!

Building this code should return a warning: field `inner` is never read.

<!-- Solution Code Snippet -->

### Step 3: Implement the button function. 

Add an `impl` block for the `struct Button`. Add a method `is_pushed` that takes in the struct as `&self` and returns a bool, if the button is pushed. 

Now remember, the pins the buttons are connected to are configured as active low. For buttons this means, that the pin is pulled low, when the button is pushed. 

In the `nrf-hal` you can find a method to check if a single pin is low. To use it, you have to add the following line to your `nrf52840_hal` imports: `prelude::InputPin`.


<!-- Solution Code Snippet -->

### Step 4: Bring up the pins!

Go to `pub fn init()`, the function that initializes the board's peripherals. Get your notes for the pin numbers that are reserver for the buttons. Configure each pin as degraded, pull-up input pin and bind it to a variable that makes it clear what button number it is connected to. 

Building this code brings up warnings about unused variables. 
<!-- Solution Code Snippet -->

### Step 5: Add everything to the board struct. 

In the definition of the board struct add a field for the `struct Buttons`
In the pub `fn init()` function. Add the button field to the instantiation of the Board struct, assigning the pins you defined earlier to the respective buttons. 

<!-- Solution Code Snippet -->

### Step 6: Run the example!

Go to `/down-the-stack/apps`

Run the following command:

```shell
cargo run --bin button
```