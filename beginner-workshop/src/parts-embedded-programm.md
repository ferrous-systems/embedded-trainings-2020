# Parts of an Embedded Program

Open the `beginner/apps` folder in VS Code.

``` console
$ # or use "File > Open Folder" in VS Code
$ code beginner/apps
```

Then open the `src/bin/hello.rs` file.

If you find it more convenient to open the repository at the root then please also add the `beginner/apps` folder to the VS Code workspace: right click the left side panel, select "Add folder to workspace" and add the `beginner/apps` folder.

Note the differences between this embedded program and a desktop program:

 The `#![no_std]` attribute indicates that the program will not make use of the standard library, `std` crate. Instead it will use the `core` library, a subset of the standard library that does not on a underlying operating system (OS).

The `#![no_main]` attribute indicates that the program will use a custom entry point instead of the default `fn main() { .. }` one.

The `#[entry]` attribute declares the custom entry point of the program. The entry point must be a divergent function; note that the return type is the never type `!`. The function is not allowed to return; therefore the program is not allowed to terminate.
