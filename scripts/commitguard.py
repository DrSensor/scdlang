#!/usr/bin/env python
from sys import stderr
from subprocess import check_output

FAIL = "\033[91m"
EC = "\033[0m"

lines = check_output("git status --porcelain", shell=True).decode("utf").split("\n")
lines = list(filter(lambda line: not ("??" in line or line == ""), lines))
untracked_changes = "\n‣ ".join(
    [line.split()[1] for line in lines if len(line.split()[0]) >= 2]
)

for line in lines:
    stats = line.split()
    if len(stats[0]) >= 2:
        print(f"{FAIL}Please stage this files:{EC}\n‣ {untracked_changes}", file=stderr)
        exit(-1)
