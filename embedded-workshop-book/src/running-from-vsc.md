# Running the Program

✅ Open the `src/bin/hello.rs` file and click the "Run" button that's hovering over the `main` function.

> Note: you will get the "Run" button if the Rust analyzer's workspace is set to the `beginner/apps` folder. This will be the case if the current folder in VS code (left side panel) is set to `beginner/apps`.

If you are not using VS code, you can run the program out of your console.
Enter the command `cargo run --bin hello` from within the `beginer/apps` folder. Rust Analyzer's "Run" button is a short-cut for that command.

> NOTE: If you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear.

Expected output:

``` console
$ cargo run --bin hello
    Running `probe-run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/hello`
(HOST) INFO  flashing program (4 pages / 16.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
INFO:hello -- Hello, world!
────────────────────────────────────────────────────────────────────────────────
(HOST) INFO  device halted without error
```

`cargo run` will compile the application and then invoke the `probe-run` tool with its argument set to the path of the output ELF file.

The `probe-run` tool will
- flash (load) the program on the microcontroller
- reset the microcontroller to make it execute the new program
- collect logs from the microcontroller and print them to the console
- print a backtrace of the program if the halt was due to an error. 

Should you need to configure the `probe-run` invocation to e.g. flash a different microcontroller you can do that in the `.cargo/config.toml` file.

``` toml
[target.thumbv7em-none-eabihf]
runner = "probe-run --chip nRF52840_xxAA" # <- add/remove/modify flags here
# ..
```

**🔎 How does flashing work?**

The flashing process consists of the PC communicating with a second microcontroller on the nRF52840 DK over USB (J2 port). This second microcontroller, named J-Link, is connected to the nRF52840 through a electrical interface known as SWD. The SWD protocol specifies procedures for reading memory, writing to memory, halting the target processor, reading the target processor registers, etc.

**🔎 How does logging work?**

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to run out of space in its logging ring buffer; this causes old log messages to be overwritten.
