# Write the Uarte implementation
## Step-by-Step Solution
* add field to the board struct
* add struct for the instance, how to figure out what the type of the inner field is
* create instance in init, add baudrate, parity etc. 
* add to instantiation of board struct
* impl fmt::Write for the Uarte struct, simple write does not work because of dma 
* example code with button is not a good idea for the simple button implementation.

I think this is plenty for an hour. 