# Next Steps

We have covered only a few of the core features of the RTIC framework but the framework has many more features like *software* tasks, tasks that can be spawned by the software; message passing between tasks; and task scheduling, which allows the creation of periodic tasks. We encourage to check the [RTIC book][rtic-book] which describes the features we haven't covered here.

[rtic-book]: https://rtic.rs/0.5/book/en/

[`usb-device`] is a library for building USB devices. It has been built using traits (the pillar of Rust's *generics*) such that USB interfaces like HID and TTY ACM can be implemented in a device agnostic manner. The device details then are limited to a trait *implementation*. There's a work in progress implementation of the `usb-device` trait for the nRF52840 device in [this PR] and there are many `usb-device` "classes" like [HID] and [TTY ACM] that can be used with that trait implementation.

[this PR]: https://github.com/nrf-rs/nrf-hal/pull/144
[HID]: https://crates.io/crates/usbd-hid
[TTY ACM]: https://crates.io/crates/usbd-serial

[`usb-device`]: https://crates.io/crates/usb-device

