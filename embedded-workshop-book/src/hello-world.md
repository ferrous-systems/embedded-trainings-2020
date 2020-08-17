# Hello, world!

In this section, we'll set up the integration in VS Code and run the first program.

✅ Open the `advanced/firmware` folder in VS Code and open the `src/bin/hello.rs` file from the `advanced/apps` folder.

> Note: To ensure full Rust-Analyzer support, do not open the whole `embedded-trainings-2020` folder.

Give Rust Analyzer some time to analyze the file and its dependency graph. When it's done, a "Run" button will appear over the `main` function. If it doesn't appear on its own, type something in the file, delete and save. This should trigger a re-load.

✅ Click the "Run" button to run the application on the microcontroller.

If you are not using VS code run the `cargo run --bin hello` command from the `advanced/firmware` folder.

> NOTE if you run into an error along the lines of "Debug power request failed" retry the operation and the error should disappear


The `firmware` workspace has been configured to cross-compile applications to the ARM Cortex-M architecture and then run them using the `probe-run` custom Cargo runner. The `probe-run` tool will load and run the embedded application on the microcontroller and collect logs from the microcontroller.

The `probe-run` process will terminate when the microcontroller enters the "halted" state. From the embedded application, one can enter the "halted" state using the `asm::bkpt` function. For convenience, an `exit` function is provided in the `dk` Hardware Abstraction Layer (HAL). This function is divergent like `std::process::exit` (`fn() -> !`) and can be used to halt the device and terminate the `probe-run` process.

Note that when the `probe-run` tool sees the device enter the halted state it will proceed to *reset-halt* the device. This is particularly important when writing USB applications because simply leaving the device in a halted state will make it appear as an unresponsive USB device to the host. Some OSes (e.g. Linux) will try to make an unresponsive device respond by power cycling the entire USB bus -- this will cause all other USB devices on the bus to be re-enumerated. Reset-halting the device will cause it to be electrically disconnected from the host USB bus and avoid the "power cycle the whole USB bus" scenario.
