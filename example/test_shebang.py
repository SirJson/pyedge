#!/usr/local/bin/pyedge

from pathlib import Path
import sys

print(Path.home())
print(f'Hello world from:\n\nPython {sys.version}\n\nYour home directory is {Path.home()}')
