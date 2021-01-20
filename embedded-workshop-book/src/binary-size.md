# Binary Size

ELF files contain metadata like debug information so their size on disk is not a good indication of the amount of Flash the program will use once it's loaded on the target device's memory.

To display the amount of Flash the program will occupy on the target device use the `cargo-size` tool, which is part of the `cargo-binutils` package.

✅ Use the following command to print the binary's size in system V format.

``` console
$ cargo size --bin hello -- -A
```

Expected output:
The breakdown of the program's static memory usage per *linker section*.

``` console
hello  :
section              size        addr
.vector_table         256         0x0
.text                9740       0x100
.rodata              4568      0x270c
.data                   8  0x20000000
.bss                 2124  0x20000008
.uninit                 0  0x20000854
```

**🔎 More details about each linker section:**

The first three sections are contiguously located in Flash memory -- Flash memory spans from address `0x0000_0000` to `0x0010_0000` (1 MB).
* The `.vector_table` section contains the *vector table*, a data structure required by the Cortex-M ISA. 
* The `.text` section contains the instructions the program will execute. 
* The `.rodata` section contains constants like strings literals. 

The next three sections, `.data`, `.bss` and `.uninit`, are located in RAM -- RAM memory spans the address range `0x2000_0000` - `0x2004_0000` (256 KB). These sections contain statically allocated variables (`static` variables).
