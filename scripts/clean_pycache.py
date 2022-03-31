#!/usr/bin/env python3
from sys import path, pycache_prefix
from os.path import join, isdir
from glob import iglob, glob
from shutil import rmtree

if pycache_prefix:
    rmtree(pycache_prefix)

for path in filter(isdir, path):
    print("Clean", path)
    pattern = join(path, "**", "__pycache__")
    for pycache_dir in iglob(pattern, recursive = True):
        try:
            rmtree(pycache_dir)
        except:
            print("Failed to Clean", path)
            break

# print(glob(join("/**/*.py[co]"), recursive = True))
# find / -type f -name "*.py[co]" -delete
# find / -type d -name "__pycache__" -delete
# find / -type d -name "__pycache__" -exec rm -r {}/* +
# python3 -m compileall 