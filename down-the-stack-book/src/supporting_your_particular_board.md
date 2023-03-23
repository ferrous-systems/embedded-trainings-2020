# Supporting your particular board

---

## Using a 'normal' PC

* Did you tell your PC it had a mouse plugged in?
* Did you tell it what I/O address the video card was located at?
* No! It auto-discovers all of these things.
  * USB, PCI-Express, SATA all have "plug-and-play"

---

## Using an Embedded System

* There is no plug-and-play
* Your MCU can put different functions (UART, SPI, etc) on different pins
* The choice of which function goes on which pin was decided by the PCB designer
* You now have to tell the software how the PCB was laid out
  * i.e UART0 TX is on Port 0, Pin 13

---

## Board Support Crates

* You can wrap this up into a Board Support Crate
* Especially useful if you are using a widely available dev-kit
  * e.g. the nRF52840-DK, or the STM32 Discovery
* Still useful if the board design is an in-house one-off
* Create the drivers and does the pin assignments for you
* Helps make your application portable across different boards

---

## Using a Board Support Crate

```rust

```

---