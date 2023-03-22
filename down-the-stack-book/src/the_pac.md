# The Peripheral Access Crate

---

## Introduction

This crate sits at the bottom of the 'stack'. It provides access to the
memory-mapped peripherals in your MCU.

---

## Memory Mapped Peripherals

* e.g. a UART peripheral
* Has registers, represented by a memory address
* Registers are usually consecutive in memory (not always)
* Peripherals can have instances (same layout of registers, different start address)
    * UART0, UART1, etc

---

## Datasheets

* *Registers* are comprised of one or more *fields*.
* Each field is at least 1 bit in length.
* Sometimes fields can only take from a limited set of values
* This is all in your datasheet!

---

## C Code!

Embedded Code in C often uses shifts and bitwise-AND to make up registers from
fields.

```c,no_run
#define UARTE_INTEN_CTS_SHIFT (0)
#define UARTE_INTEN_CTS_MASK (0x00000001)
#define UARTE_INTEN_NCTS_SHIFT (1)
#define UARTE_INTEN_NCTS_MASK (0x00000001)
#define UARTE_INTEN_RXRDY_SHIFT (2)
#define UARTE_INTEN_RXRDY_MASK (0x00000001)

// The other eight fields are skipped for brevity
uint32_t cts = 0;
uint32_t ncts = 1;
uint32_t rxrdy = 1;

uint32_t inten_value = ((cts & UARTE_INTEN_CTS_MASK) << UARTE_INTEN_CTS_SHIFT)
    | ((ncts & UARTE_INTEN_NCTS_MASK) << UARTE_INTEN_NCTS_SHIFT)
    | ((rxrdy & UARTE_INTEN_RXRDY_MASK) << UARTE_INTEN_RXRDY_SHIFT);

*((volatile uint32_t*) 0x40002300) = inten_value;
```

---

## Adding structure

The various registers for a peripheral are often grouped into a `struct`

```c,no_run
typedef volatile struct uart0_reg_t {
    uint32_t tasks_startrx; // @ 0x000
    uint32_t tasks_stoprx; // @ 0x004
    // ...
    uint32_t inten; // @ 0x300
    uint32_t _padding[79]; 
    uint32_t baudrate; // @ 0x500
} uart0_reg_t;
```
---

## Rust Code

You *could* do this in Rust if you wanted...

```rust,no_run
const UARTE0_INTEN: *mut u32 = 0x4000_2300 as *mut u32;
unsafe { UARTE0_INTEN.write_volatile(0x0000_0003); }
```

But it seems like a lot of reading PDFs and re-typing everything?

---

## CMSIS-SVD Files

A CMSIS-SVD (or just SVD) file is an XML description of all the peripherals,
registers and fields on an MCU.

We can use `svd2rust` to turn this into a Peripheral Access Crate.

```mermaid
graph LR
    svd[(SVD XML)] --> svd2rust[<tt>svd2rust</tt>] --> rust[(Rust Source)]
```

---

## The `svd2rust` generated API

* The crate has a top-level `struct Peripherals` with members for each *Peripheral*
* Each *Peripheral* gets a `struct`, like `UARTE0`, `SPI1`, etc.
* Each *Peripheral* `struct` has members for each *Register*
* Each *Register* gets a `struct`, like `BAUDRATE`, `INTEN`, etc.
* Each *Register* `struct` has `read()`, `write()` and `modify()` methods

```mermaid
graph TB
    Peripherals --> uarte1[.UARTE1: <b>UARTE1</b>]
    uarte1 --> uart1_baudrate[.baudrate: <b>BAUDRATE</b>]
    uarte1 --> uart1_inten[.inten: <b>INTEN</b>]
    Peripherals --> uarte2[.UARTE2: <b>UARTE2</b>]
    uarte2 --> uart2_baudrate[.baudrate: <b>BAUDRATE</b>]
    uarte2 --> uart2_inten[.inten: <b>INTEN</b>]
```

---

## The `svd2rust` generated API (2)

* The `read()` method returns a special proxy object, with methods for each *Field*
* The `write()` method takes a closure, which is given a special 'proxy' object, with methods for each *Field*
  * All the *Field* changes are batched together and written in one go
  * Any un-written *Fields* are set to a default value
* The `modify()` method gives you both
  * Any un-written *Fields* are left alone

---

## An example

```rust,no_run
// nrf52840 is the PAC
let p = nrf52840::Peripherals::take().unwrap();
// This register has only one field
let current_baud_rate = p.UARTE1.baudrate.read().baudrate();
// This register has multiple fields
p.UARTE1.inten.modify(|_r, w| {
    w.cts().enabled();
    w.ncts().enabled();
    w.rxrdy().enabled();
    w    
});
```

---

## Wait, what's a closure?

* It's an anonymous function, declared in-line with your other code
* It can 'capture' local variables (although we don't use that feature here)
* It enables a very powerful Rust idiom, that you can't easily do in C...

---

## Let's take it in turns

* I, the callee, need to set some stuff up
* You, the caller, need to do a bit of work
* I, the callee, need to clean everything up

We can use a closure to insert the caller-provided code in the middle of our
function. We see this used
[all](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.map)
[over](https://doc.rust-lang.org/core/primitive.str.html#method.matches)
[the](https://doc.rust-lang.org/std/thread/fn.spawn.html) Rust standard library!

---

## You tell me...

What are the three steps here?

```rust
p.UARTE1.inten.modify(|_r, w| {
    w.cts().enabled();
    w.ncts().enabled();
    w.rxrdy().enabled();
    w    
});
```

---

## Documentation

Docs can be generated from the source code.

See <https://docs.rs/nrf52840>

Note that `uarte0` is a *module* and `UARTE0` could mean either a `struct` type,
or a field on the `Peripherals` struct.
