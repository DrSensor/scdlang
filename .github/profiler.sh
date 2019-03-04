#!/bin/sh
# TODO: make it as dedicated gh-action when published to marketplace
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

mkdir -p ${HOME}/.perf

bin=`sh -c "echo $*"`
time -f "$json" -o "${HOME}/.perf/${GITHUB_ACTION}.json" $bin 1>/dev/null

cat "${HOME}/.perf/${GITHUB_ACTION}.json"
