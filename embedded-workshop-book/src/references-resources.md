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

- [Slides used during the workshop](./Advanced_Embedded_Training.pdf)
- [nRF52840 Product Specification 1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
- [Universal Serial Bus (USB) Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)

## When's The Next Workshop?
If you've enjoyed this workshop and would like to join us for another one, [subscribe to our newsletter](https://ferrous-systems.us19.list-manage.com/subscribe/post?u=94954b16eab55b59525c890cb&id=5eaf5b14e6)! This is where we'll announce upcoming public courses.

If your company would like a training with custom content, or a private run of an existing workshop, we'll be happy to make this happen for you. Get in touch with us [via phone or E-Mail](https://ferrous-systems.com/#contact) for further information.

brought to you with 🧡 by

<img src="./logo-monochrome.svg" alt="ferrous systems logo" width=300px>
