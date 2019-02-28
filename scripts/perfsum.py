#!/usr/bin/env python
import json
import requests
import io
import matplotlib
from itertools import chain
from textwrap import shorten
from sys import stdin
from matplotlib import pyplot as plt, get_backend as mplbackend

plt.style.use('bmh')
# ----------------------- helper -----------------------
def filter_by(command, perf_data, fillempty=0):
    return list(
        map(lambda perf: next(
            map(lambda log: log['time'],
                filter(lambda log: log['exec'] == command,
                    perf
                )
            )
        ) if len(perf) > 0 else fillempty, perf_data)
    )
# ------------------------------------------------------

data = json.load(stdin)

subjects = [ n['subject'] for n in data ]
perfs = [ n['perf'] for n in data ]
commands = list(set( # get and flatten perf data then dedupe
    map(lambda p: p['exec'],
        chain.from_iterable(
            filter(lambda p: len(p) != 0, perfs)
        )
    )
))

max_char_title = max([ len(c) for c in commands ])

fig, axs = plt.subplots(len(commands), 1)
fig.set_figwidth(max_char_title/5)
fig_height = fig.get_figwidth()

for i, command in enumerate(commands):
    times = filter_by(command, perfs)
    xy = list(filter(lambda p: p[0] != 0, zip(times, subjects)))

    x = list(map(lambda p: p[0], xy))
    y = list(map(lambda p: shorten(p[1], 25, placeholder='...'), xy))

    axs[i].barh(y, x)

    fig_height += len(xy) / fig.get_figwidth()
    axs[i].invert_yaxis()
    axs[i].set_xlabel('Exec Time (s)')
    axs[i].set_title(command)

fig.set_figheight(fig_height)
fig.tight_layout()

if mplbackend() == 'agg':
    plt.savefig('perf.png',
                aspect='auto',
                transparant=True,
                dpi=300)
    response = requests.post('https://vgy.me/upload', files={'file': open('perf.png', 'rb')})
    url = response.json()['image']
    print(f'![perf result]({url})')
else:
    plt.show()
