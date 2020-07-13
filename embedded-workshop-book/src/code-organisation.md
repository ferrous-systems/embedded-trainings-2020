# Code Organization

The `advanced` folder contains both "host" code, code that will run on the host, and "firmware" code, code that will run on the nRF52840 SoC. "host" and "firmware" source code has been placed in different Cargo workspaces so that each can be compiled with different compilation profiles. The `host` workspace will be natively compiled, whereas the `firmware` workspace will be cross-compiled for the ARM Cortex-M architecture.

``` console
$ cd advanced

$ tree -L 1 .
.
├── common
├── firmware
├── host
└── README.md
```

In addition to these two workspaces there's a third folder called "common". This folder contains `no_std` code that can be depended on by either "host" code or "firmware" code.