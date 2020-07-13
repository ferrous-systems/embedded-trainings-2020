# Binary Size

ELF files contain metadata like debug information so their size on disk is not a good indication of the amount of Flash the program will use once it's loaded on the target device's memory.

To display the amount of Flash the program will occupy on the target device use the `cargo-size` tool (part of the `cargo-binutils` package):

``` console
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

This gives you a breakdown of the program's static memory usage per *linker section*.

The `.vector_table` section contains the *vector table*, a data structure required by the Cortex-M ISA. The `.text` section contains the instructions the program will execute. The `.rodata` section contains constants like strings literals. These three sections are contiguously located in Flash memory -- Flash memory spans from address `0x0000_0000` to `0x0010_0000` (1 MB).

The next three sections, `.data`, `.bss` and `.uninit`, are located in RAM -- RAM memory spans the address range `0x2000_0000` - `0x2004_0000` (256 KB). These sections contain statically allocated variables (`static` variables).

Another other useful tool to analyze the binary size of a program is `cargo-bloat`:

``` console
$ cargo bloat --bin hello
File  .text   Size      Crate Name
0.7%  13.5% 1.3KiB        std <char as core::fmt::Debug>::fmt
0.5%   9.6%   928B      hello hello::__cortex_m_rt_main
0.4%   8.4%   804B        std core::str::slice_error_fail
0.4%   8.0%   768B        std core::fmt::Formatter::pad
0.3%   6.4%   614B        std core::fmt::num::<impl core::fmt::Debug for usize>::fmt
(..)
5.1% 100.0% 9.4KiB            .text section size, the file size is 184.5KiB
```

This breakdowns the size of the `.text` section by function. This breakdown can be used to identify the largest functions in the program; those could then be modified to make them smaller.
