#!/bin/sh
set -e

json='{
  "command": "%C",
  "memory": {
    "peak": %M,
    "average": %K,
    "set": %t,
    "data": %D,
    "stack": %p,
    "text": %X
  },
  "cpu": "%P",
  "io": {
    "fs": {
      "input": %I,
      "output": %O
    },
    "sig": %k,
    "socket": {
      "recv": %r,
      "send": %s
    }
  },
  "return": "%x"
}'

if [ -n "$2" ]; then
  mkdir -p $(dirname "${*:2}")
  time -f "$json" -o "${*:2}" $1 1>/dev/null
else
  time -f "$json" $1 2>&1 1>/dev/null
fi
