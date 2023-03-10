# PAC Exercise

In this exercise you will generate a PAC (Peripheral Access Crate) from an svd file. 

Install `svd2rust` via cargo 
Download https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/nrf52.svd (or some tagged version)

Run svd2rust with the SVD file to generate a PAC using the `cortex-m` flag
Notice how it's one long line of text in the source file. (mine is clean?!)
Look at the PAC docs with cargo doc --open. (this needs a cargo.toml, make one yourself??, what do we want folks to see?)
cargo fmt the crate. No change to the docs, but a bit more readable.
cargo install form, use form to process the one-big-file into one-file-per-module. 
form -i src/lib.rs -o src/  Re-run cargo fmt. 
Find the definition of the ENABLE register for UARTE0, in the PDF datasheet and in the SVD file, and in the svd2rust generated code.
Write a simple program which uses the PAC to enable the UART. See how writing arbitrary values to the ENABLE field in the ENABLE register is unsafe, because only values 0 or 8 should be used.

base address Uarte0 0x40002000
enable UARTE 0x500 

add pac to cargo.toml