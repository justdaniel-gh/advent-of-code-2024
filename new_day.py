#!/bin/env python3

from glob import glob
from os import system, makedirs
from shutil import copy
from pathlib import Path

try:
    day_num = sorted(int(d.replace("day", "")) for d in glob("day*")).pop() + 1
except:
    day_num = 1
    makedirs("puzzles", exist_ok=True)

new_day = "day{}".format(day_num)

system(f"cargo init {new_day}")
system(f"cd {new_day} && cargo add --path ../utils utils")

with open(Path("puzzles") / f"{new_day}.txt", "a"):
    pass

with open(Path("puzzles") / f"{new_day}_test1.txt", "a"):
    pass

with open(Path("puzzles") / f"{new_day}_test2.txt", "a"):
    pass

with open(Path(f"{new_day}") / f"puzzle.txt", "a"):
    pass


with open("template.rs_", "r") as fd:
    with open(Path(f"{new_day}") / "src" / "main.rs", "w") as ofd:
        ofd.write(fd.read().replace(r"{{DAY_NUM}}", str(day_num)))