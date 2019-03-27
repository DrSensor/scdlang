#!/usr/bin/env python
from secrets import token_urlsafe
import re
import sys


def random(num_of_char):
    random_chars = token_urlsafe(num_of_char).replace("_", "")
    return re.sub(r"\W+", "", random_chars)


for x in range(int(sys.argv[1])):
    current_state = f"A{random(1)}"
    next_state = f"B{random(1)}"
    event = f"C{random(1)}"
    print(f"{current_state} -> {next_state} {trigger}")
