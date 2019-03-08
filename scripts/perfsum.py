#!/usr/bin/env python
import json
import requests
from itertools import chain
from textwrap import shorten
from sys import stdin
from matplotlib import pyplot as plt, get_backend as mplbackend

plt.style.use("bmh")
x_axis = {"time": "Exec Time (s)", "memory": "Peak Memory (kB)", "cpu": "Load CPU (%)"}


def to_number(var):
    if type(var) == str:
        if "?" in var:
            return 0
        elif "%" in var:
            return int(var.strip("%"))
    else:
        return var


def max_number(var):
    if type(var) == str:
        if "(%)" in var:
            return 100
    else:
        return None


def filter_by(command, perf_data, keys, fillempty=0):
    result = []
    for key, title in keys.items():
        _ = list(
            map(
                lambda perf: next(
                    map(
                        lambda log: to_number(log[key]),
                        filter(lambda log: log["exec"] == command and key in log, perf),
                    )
                )
                if len(perf) > 0
                else fillempty,
                perf_data,
            )
        )
        if len(_) != 0:
            result.append((_, title, max_number(title)))
    return result


data = json.load(stdin)

subjects = [n["subject"] for n in data]
perfs = [n["perf"] for n in data]
commands = list(
    set(  # get and flatten perf data then dedupe
        map(
            lambda p: p["exec"],
            chain.from_iterable(filter(lambda p: len(list(p)) != 0, perfs)),
        )
    )
)

max_char_title = max([len(c) for c in commands])
max_perf_keys = max(map(lambda p: len(p.keys()), chain.from_iterable(perfs)))

fig = plt.figure()
gs = fig.add_gridspec(len(commands), max_perf_keys - 1)
fig.set_figwidth(max_char_title / 5)
fig_height = fig.get_figwidth()

for i, command in enumerate(commands):
    cmd_perfs = filter_by(command, perfs, x_axis)
    first_ax = None
    for j, (results, title, limit) in enumerate(cmd_perfs):
        xy = list(
            filter(
                lambda p: p[0] != 0 if len(cmd_perfs) == 1 else True,
                zip(results, subjects),
            )
        )

        x = list(map(lambda p: p[0], xy))
        y = list(map(lambda p: shorten(p[1], 25, placeholder="..."), xy))

        grid = gs[i, j] if len(cmd_perfs) > 1 else gs[i, :]
        ax = fig.add_subplot(grid, sharey=first_ax)

        if first_ax is not None:
            ax.invert_yaxis()
            ax.set_title("/".join(command.split("/")[-2:]), loc="right")
            plt.setp(ax.get_yticklabels(), visible=False)
        elif len(cmd_perfs) == 1:
            ax.invert_yaxis()
            ax.set_title(command, loc="right")

        ax.barh(y, x)
        ax.set_xlim(right=limit)

        fig_height += len(xy) / fig.get_figwidth()
        ax.set_xlabel(title)
        first_ax = ax if j == 0 else None

fig.set_figheight(fig_height)

if mplbackend() == "agg":
    plt.savefig("perf.png", aspect="auto", transparant=True, dpi=300)

    response = requests.post(
        "https://api.cloudinary.com/v1_1/perf/image/upload",
        files={"file": open("perf.png", "rb"), "upload_preset": (None, "perf-storage")},
    )
    url = response.json()["secure_url"]
    print(f"![perf result]({url})")
else:
    plt.show()
