# Tooling check

## Setup check

✅ First, let's check that you have installed all the required tools.

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

## More tools

✅ Now let's install some tools shipped with the workshop material.

### Beginner workshop

From the `tools` folder run these commands *from different terminals so they'll run in parallel*:

- `cargo install --path dk-run`
- `cargo install --path usb-list`
- `cargo install --path dongle-flash`
- `cargo install --path serial-term`
- `cargo install --path change-channel`

Leave the processes running in the background.

### Advanced workshop

From the `tools` folder run these commands *from different terminals so they'll run in parallel*:

- `cargo install --path dk-run`
- `cargo install --path usb-list`

Leave the processes running in the background.
