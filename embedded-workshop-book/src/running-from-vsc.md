# Running the Program from VS Code

Both `cargo-embed` and `cargo-flash` are tools based on the `probe-rs` library. This library exposes an API to communicate with the J-Link and perform all the operations exposed by the JTAG protocol. We have developed a small Cargo runner called [probe-run](https://github.com/knurling-rs/probe-run) that uses the `probe-rs` library to streamline the process of running a program and printing logs, like `cargo-embed`, while also having better integration into VS code. We'll be using it in this workshop, and you can utilize it in your future projects too.

✅ Open the `src/bin/hello.rs` file and click the "Run" button that's hovering over the `main` function.

> Note: you will get the "Run" button if the Rust analyzer's workspace is set to the `beginner/apps` folder. This will be the case if the current folder in VS code (left side panel) is set to `beginner/apps`.

If you are not using VS code, you can run the program out of your console.
Enter the command `cargo run --bin hello` from within the `beginer/apps` folder. Rust Analyzer's "Run" button is a short-cut for that command.

Expected output:

``` console
$ cargo run --bin hello
INFO:hello -- Hello, world!
stack backtrace:
   0: 0x0000229c - __bkpt
   1: 0x0000030e - hello::__cortex_m_rt_main
   2: 0x0000011a - main
   3: 0x00001ba2 - Reset
```

`cargo run` will compile the application and then invoke `probe-run` with its argument set to the path of the output ELF file.

Unlike `cargo-embed`, `probe-run` will terminate when the program reaches a breakpoint (`asm::bkpt`) that halts the device. Before exiting, `probe-run` will print a stack backtrace of the program starting from the breakpoint. This can be used to write small test programs that are meant to perform some work and then terminate.
