# `dongle-flash` not working

``` console
$ dongle-flash loopback.hex
packaging iHex using nrfutil ...
Error: No such file or directory (os error 2)
```

this indicates that `nrfutil`, the Python tool, is not installed or not available in your PATH. Instructions on how to install `nrfutil` can be found in the [`nrfutil` section of the Installation Instructions].

❗️ If you install `nrfutil` in a virtual environment you'll need to activate the environment; the `nrfutil` binary must be available in your PATH.

[`nrfutil` section of the Installation Instructions]: ./installation.md#nrfutil