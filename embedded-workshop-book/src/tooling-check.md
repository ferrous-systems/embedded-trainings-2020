# Tooling check

## Setup check

✅ Let's check that you have installed all the tools listed in the previous section.

❗ The first two commands *must* return version `0.8.x`

``` console
$ cargo flash --version
0.8.0

$ cargo embed --version
0.8.0

$ cargo size --version
cargo-size 0.3.0

$ nrfutil version
nrfutil version 6.1.0
```

✅ Now let's make sure you've installed the tools shipped with the workshop material.

### Beginner workshop

Run the commands listed here and see if they produce similar output, i.e. *don't* yield `command not found: ...`

```console
$ usb-list
Bus 020 Device 007: ID 1b1c:0a42
Bus 020 Device 006: ID 1fc9:0132
(..)

$ dongle-flash
Error: expected exactly one argument

$ serial-term
(waiting for the Dongle to be connected)

$ change-channel
Error: expected exactly one argument
```

### Advanced workshop

Run the commands listed here and see if they produce similar output, i.e. *don't* yield `command not found: ...`

```console
$ usb-list
Bus 020 Device 007: ID 1b1c:0a42
Bus 020 Device 006: ID 1fc9:0132
(..)
```