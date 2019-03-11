#!/bin/sh
# TODO: make it as dedicated gh-action when published to marketplace
# 1. Add environment variable to set command alias
# 2. Add ${HOME}/.bin to PATH
# 3. Inheret exit code from args, not `time`
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

# TODO: use looping
for cmd in "$@"; do
  echo "Running '$cmd'..."
  bin=`sh -c "echo $cmd"`
  if time -f "$json" --append -o "${HOME}/.perf/${GITHUB_ACTION}.json" $bin 1>/dev/null; then
    echo "Successfully ran '$cmd'"
    echo
  else
    echo
    exit_code=$?
    echo "Failure running '$cmd', exited with $exit_code"
    exit $exit_code
  fi
done

cat "${HOME}/.perf/${GITHUB_ACTION}.json"
