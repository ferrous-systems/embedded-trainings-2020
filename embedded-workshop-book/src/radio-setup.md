# Radio Setup

✅ Open the `src/bin/radio-send.rs` file.

✅ First run the program `radio-send.rs` as it is. You should see new output in the output of `cargo xtask serial-term`.

``` console
$ cargo xtask serial-term
deviceid=588c06af0877c8f2 channel=20 TxPower=+8dBm app=loopback.hex
received 5 bytes (LQI=49)
```

The program broadcasts a radio packet that contains the 5-byte string `Hello` over channel 20 (which has a center frequency of 2450 MHz). The `loopback` program running on the Dongle is listening to all packets sent over channel 20; every time it receives a new packet it reports its length and the Link Quality Indicator (LQI) metric of the transmission over the USB/serial interface. As the name implies the LQI metric indicates how good the connection between the sender and the receiver is.

