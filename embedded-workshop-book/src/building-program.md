# Building an Embedded Program

The following command cross compiles the program to the ARM Cortex-M4 architecture.

``` console
$ cargo build --bin hello
```

The default in a Cargo project is to compile for the host (native compilation) but the `beginner/apps` project has been configured for cross compilation. This configuration can be seen in the Cargo configuration file (`.cargo/config`):

``` text
# .cargo/config
[build]
target = "thumbv7em-none-eabi" # = ARM Cortex-M4
```

The output of the compilation process will be an ELF (Executable and Linkable Format) file. The file will be placed in the `target/thumbv7em-none-eabi` directory.

``` console
$ file target/thumbv7em-none-eabi/debug/hello
hello: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped
```
