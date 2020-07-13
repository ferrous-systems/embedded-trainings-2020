# Viewing Logs

To observe the program logs you can use the `cargo-embed` tool.

``` console
$ cargo embed --bin hello
```

This command will bring up a Text User Interface (TUI). You should see "Hello, world!" in the output. You can close the interface using Ctrl-C.

`cargo-embed` has no `--chip` flag; instead the target chip needs to be specified in a file named `Embed.toml`. This file must be placed in the root of the Cargo project / workspace, next to the `Cargo.toml` file.

``` toml
# Embed.toml
[general]
chip = "nRF52840_xxAA"
```

Logging is implemented using the Real Time Transfer (RTT) protocol. Under this protocol the target device writes log messages to a ring buffer stored in RAM; the PC communicates with the J-Link to read out log messages from this ring buffer. This logging approach is non-blocking in the sense that the target device does not have to wait for physical IO (USB comm, serial interface, etc.) to complete while logging messages since they are written to memory. It is possible, however, for the target device to run out of space in its logging ring buffer; this causes old log messages to be overwritten.
