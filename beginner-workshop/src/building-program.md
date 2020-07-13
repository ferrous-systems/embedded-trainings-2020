# Building an Embedded Program


The following command cross compiles the program to the ARM Cortex-M4 architecture. The `--target` arguments instructs Cargo to cross compile the program.

``` console
$ cargo build --target thumbv7em-none-eabi --bin hello
```

The default in a new Cargo project is to compile for the host (native compilation). Within the `beginner/apps` folder you can however omit the `--target ` flag and Cargo will still cross compile for the ARM Cortex-M4 architecture.

``` console
$ cargo build --bin hello
```

The reason for this is that the default compilation target has been set to ARM Cortex-M4 in the Cargo configuration file (`.cargo/config`):

``` text
# .cargo/config
[build]
target = "thumbv7em-none-eabi"
```

The output of the compilation process will be an ELF (Executable and Linkable Format) file. The file will be placed in the `beginner/apps/target` directory. To display the amount of Flash the program will occupy on the target device use the `rust-size` tool (part of the `cargo-binutils` package):


``` console
$ rust-size target/thumbv7em-none-eabi/debug/hello
   text    data     bss     dec     hex filename
  14564       8    2124   16696    4138 target/thumbv7em-none-eabi/debug/hello
```

`14460` bytes is the amount of Flash memory the program will occupy.

Alternatively, you can run the `cargo-size` subcommand, which will build the program before displaying the size of the binary.

``` console
$ cargo size --bin hello
   text    data     bss     dec     hex filename
  14564       8    2124   16696    4138 hello
```

Passing the `-A` flag to `rust-size` or `cargo-size` will give a more detailed breakdown of the static memory usage:

``` console
$ # omit the `--` flag if using `rust-size`
$ cargo size --bin hello -- -A
hello  :
section              size        addr
.vector_table         256         0x0
.text                9740       0x100
.rodata              4568      0x270c
.data                   8  0x20000000
.bss                 2124  0x20000008
.uninit                 0  0x20000854
```

The `.vector_table` section contains the *vector table*, a data structure required by the Cortex-M ISA. The `.text` section contains the instructions the program will execute. The `.rodata` section contains constants like strings literals. These three sections are contiguously located in Flash memory -- Flash memory spans from address `0x0000_0000` to `0x0010_0000` (1 MB).

The next three sections, `.data`, `.bss` and `.uninit`, are located in RAM -- RAM memory spans the address range `0x2000_0000` - `0x2004_0000` (256 KB). These sections contain statically allocated variables (`static` variables).
