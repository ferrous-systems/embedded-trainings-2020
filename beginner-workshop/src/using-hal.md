# Using a Hardware Abstraction Layer

In this section we'll start using the hardware features of the nRF52840 and the board.

Open the `src/bin/led.rs` file.

The `dk` crate / library is a Hardware Abstraction Layer (HAL) over the nRF52840 Development Kit. The purpose of a HAL is to abstract away the device-specific details of the hardware, for example registers, and instead expose a higher level API more suitable for application development.

The `dk::init` function we have been calling in all programs initializes a few of the nRF52840 peripherals and returns a `Board` structure that provides access to those peripherals. We'll first look at the `Leds` API. Open the documentation for the `dk` crate running the following command from the `beginner/apps` folder:

``` console
$ cargo doc -p dk --open
```

Check the API docs of the `Led` abstraction then run the `led` program. Two of the green LEDs on the board should turn on; the other two should stay off.

> NOTE this program will not terminate itself. Within VS code you need to click "Kill terminal" (garbage bin icon) in the bottom panel to terminate it.

Now, uncomment the `log::set_max_level` line. This will make the logs more verbose; they will now include logs from the board initialization function (`dk::init`) and from the `Led` API.

Among the logs you'll find the line "I/O pins have been configured for digital output". At this point the electrical pins of the nRF52840 microcontroller has been configured to drive the 4 LEDs on the board.

After the `dk::init` logs you'll find logs about the `Led` API. As the logs indicate an LED becomes active when the output of the pin is a *logical zero*, which is also referred as the "low" state. This "active low" configuration does not apply to all boards: it depends on how the pins have been wired to the LEDs. You should refer to the [board documentation] to find out which pins are connected to LEDs and whether "active low" or "active high" applies to it.

[board documentation]: https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52840_dk%2FUG%2Fnrf52840_DK%2Fintro.html