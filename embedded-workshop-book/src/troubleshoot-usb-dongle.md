# Dongle USB functionality is not working

> NOTE: this section only applies to the Beginner workshop

If you don't get any output from `serial-term` it could just have been that first line got lost when re-enumerating the device from bootloader mode to the loopback application. Run these *two* commands:

``` console
$ change-channel 20
requested channel change to channel 20

$ change-channel 20
requested channel change to channel 20
```

You should get *two* lines of output in `serial-term`

``` console
$ serial-term
now listening on channel 20
now listening on channel 20
```

If that's the case you are good to go, return to the ["Interference"] section.

["Interference"]: /dongle.html#interference

If you only get one line of output then your OS may be losing some serial data -- we have seen this behavior on some macOS machines. You will still be able to work through the exercises but will miss log data every now and then. Return to the ["Interference"] section.

If you don't get *any* output from `serial-term` and/or the `change-channel` command fails then the Dongle's USB functionality is not working correctly. In this case you should flash one of the `loopback-nousb*` programs (repeat the procedure: put the device in bootloader mode; then run `dongle-flash`). These programs have no USB functionality but you will be able to use them to do the exercises. There are four `.hex` files available; pick one of them and flash it but take note of the number in the file name; this is the radio channel the Dongle will listen to. The Dongle will toggle the state of its green LED when it receives a packet. Return to the ["Interference"] section.
