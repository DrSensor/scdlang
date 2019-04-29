#!/bin/sh
set -e

for cmd in "$@"; do
  echo "Running '$cmd'..."
  if sh -c "$cmd"; then
    # no op
    echo
    echo "Successfully ran '$cmd'"
  else
    exit_code=$?
    echo
    echo "Failure running '$cmd', exited with $exit_code"
    exit $exit_code
  fi
done
