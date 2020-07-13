# References and Resources

## Beginner Material

- [nRF52840 Product Specification 1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
- The [Embedded Rust Book][embedded rust] is a great learning resource, especially the Concurrency chapter.
- If you are looking to write an interrupt handler, look at the [`#[interrupt]` attribute][interrupt]. All interrupts implemented by the nrf52840 hal are listed in [`nrf52840-pac/src/lib.rs`][pac].  
It is also recommended that you work through the advanced material of this course to learn about [RTIC][rtic].

[pac]: https://github.com/nrf-rs/nrf52840-pac/blob/9558a3ed032b2aec7e57c2f42330f1dee0000a04/src/lib.rs#L167
[interrupt]: https://docs.rs/cortex-m-rt-macros/0.1.5/cortex_m_rt_macros/attr.interrupt.html
[rtic]: https://docs.rs/cortex-m-rtic/0.5.3/rtic/
[embedded rust]: https://rust-embedded.github.io/book/



## Advanced Material

- [nRF52840 Product Specification 1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
- [Universal Serial Bus Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)
