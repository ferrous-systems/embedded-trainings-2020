# Building an Embedded Program

The default in a Cargo project is to compile for the host (native compilation). The `beginner/apps` project has been configured for cross compilation to the ARM Cortex-M4 architecture. This configuration can be seen in the Cargo configuration file (`.cargo/config`):

``` text
# .cargo/config
[build]
target = "thumbv7em-none-eabi" # = ARM Cortex-M4
```

✅ Inside the folder `beginner/apps`, use the following command to cross compile the program to the ARM Cortex-M4 architecture.

``` console
$ cargo build --bin hello
```

The output of the compilation process will be an ELF (Executable and Linkable Format) file. The file will be placed in the `target/thumbv7em-none-eabi` directory.

✅ Run `$ file target/thumbv7em-none-eabi/debug/hello` and compare if your output is as expected.

Expected output:
``` console
$ file target/thumbv7em-none-eabi/debug/hello
hello: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped
```