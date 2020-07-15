# RTIC hello

RTIC, Real Time on Integrated Circuits, is a framework for building evented, time sensitive applications.

✅ Open the `src/bin/rtic-hello.rs` file.

RTIC applications are written in RTIC's Domain Specific Language (DSL). The DSL extends Rust syntax with custom attributes like `#[init]` and `#[idle]`.

RTIC makes a clearer distinction between the application's initialization phase, the `#[init]` function, and the application's main loop or main logic, the `#[idle]` function. The initialization phase runs with interrupts disabled and interrupts are re-enabled before the `idle` function is executed.

`rtic::app` is a procedural macro that generates extra Rust code, in addition to the user's functions. The fully expanded version of the macro can be found in the file `target/rtic-expansion.rs`. This file will contain the expansion of the procedural macro for the last compiled RTIC application.

✅ Build the `rtic-hello` example and look at the generated `rtic-expansion.rs` file. 

The generated code should look like this. Note that interrupts are disabled during the execution of the `init` function:

``` rust
fn main() -> ! {
    rtic::export::interrupt::disable();
    let late = init(init::Context::new(/* .. */));
    rtic::export::interrupt::enable();
    idle(idle::Context::new(/* .. */))
}
```