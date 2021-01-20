# Tooltips

Besides the ones covered in this workshop, there are many more tools that make embedded development easier. 
Here, we'd like to introduce you to some of these tools and encourage you to play around with them and adopt them if you find them helpful!

## `cargo-bloat`

`cargo-bloat` is a useful tool to analyze the binary size of a program. You can install it through cargo:

``` console
$ cargo install cargo-bloat
(..)
Installed package `cargo-bloat v0.10.0` (..)
```

Let's inspect our beginner course's `hello` program with it:

``` console
$ cd beginner/apps
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