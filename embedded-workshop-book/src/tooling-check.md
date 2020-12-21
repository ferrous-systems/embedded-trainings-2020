# Tooling check

## Setup check

✅ First, let's check that you have installed all the tools listed in the previous section.

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

✅ Now let's make sure you've installed the tools shipped with the workshop material.

### Beginner workshop

```console
$  usb-list
Bus 020 Device 007: ID 1b1c:0a42
Bus 020 Device 006: ID 1fc9:0132
```

### Advanced workshop

From the `tools` folder run these commands *from different terminals so they'll run in parallel*:

- `cargo install --path usb-list`

Leave the processes running in the background.
