# Reading PAC Documentation
(This should be covered or at least preceded by a lecture that includes basic use of closures and the read/write/modify API)

Generate and open the PAC's docs using the following command:

```
cargo doc --open
```

In the `Structs` section look for the `struct Peripherals`. Taking ownership of it will be the first step later on. Note that only the method `steal()` is documented. It is an unsafe method, and to be avoided. `Peripherals` has a field named `UARTE0`.

In the `modules` section, look for the `uarte0` module. It is divided into submodules. `enable` is the register we are concerned about. Clicking on it shows the associated type definitions.  

* `W` - the register ENABLE writer with the following methods:
    * `enable()` returns the field ENABLE writer `ENABLE_W`. 
    * `unsafe bits()` writes raw bits into the register.
* `R` - the register ENABLE reader writer with the following methods:
    * `enable()` returns the field ENABLE reader `ENABLE_R`. 
    * `bits()` reads raw bits from the register. 

The types `ENABLE_R` and `ENABLE_W` have methods that you can use if you don't want to deal with raw bits. Check them out!

Usage: If you want to write or read something from the uarte register and you want to avoid dealing with raw bits, you first have to call a method that gives you access to the respective reader or writer, and then call the method that does what you want. 

Example:

```rust
// this reads the enable register, and returns true if the register is disabled.
uarte.enable.read().is_disabled()
```

Note the difference between the struct field `UARTE0` in `Peripherals` and the module `uarte0`.

## Finding corresponding sections in the PAC

* `dk_pac/src/lib.rs` defines the single peripherals with their register block addresses and contains a struct definition for the `struct Peripherals`. There are two methods for this struct: `take()` and `steal()`. `take()` assures, that only one instance of this can exist. Hence, it's safe. Note that `take()` is only available with the `critical-section` feature enabled.

* `dk_pac/src/uarte0.rs` defines a struct that contains all the registers of the `UARTE0` register block. The `enable` field represents the register of the same name. 

* `dk_pac/src/uarte0/enable.rs` defines the types associated with this register that you already saw in the docs. 

