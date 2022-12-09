# Tooling check

## Setup check

✅ Let's check that you have installed all the tools listed in the previous section.

``` console
$ cargo size --version
cargo-size 0.3.3
```
✅ Connect the nRF52840-DK with your computer by plugging the usb-cable into the J2 connector on the DK (the usb connector on the short side of the board). 

✅ In the terminal run the following command from the `begginer/apps` folder. This will build and run a simple program on the DK to test the set-up. 

``` console
$ cargo run --bin hello
```

