# pyedge

Like "/usr/bin/env python" but always selects the python version with the highest version number. This is useful on systems with multiple Python versions. 

## Motivation

Imagine a stable Debian system with a self-compiled Python interpreter to provide a newer version. The Python Makefile will make sure that everything is installed so that there will be no clash between the system Python and your own Python. 

If you now want to execute your scripts with a shebang like `/usr/bin/env python3` Debian would start the older Interpreter which might not support all features you need.

After hitting this problem myself I decided to write this small Rust application that automatically select the latest Python version on your system and uses that to execute your script.

Since the only dynamic dependency is libc this should work on many Linux systems out there. No dependencies have to be installed, only the `pyedge` executable is needed. If my solution is working well enough I might consider switching to `musl` to have a completely independent binary.

## Install

### Downloads

* Linux (x64): https://github.com/SirJson/pyedge/releases/tag/Beta-1

Other prebuild binaries might follow. In case you need it right now follow the steps below.

### Building from Source

Run `make` and `sudo make install` to make pyedge available as a shebang. The default install path is `/usr/local`. You can change that by modifying the `PREFIX` variable. See `./example/test_shebang.py` for details. 

Because the application is not bound to any specific path you can also use your own setup if you want.

If you want to remove pyedge run `make clean` and `sudo make uninstall`