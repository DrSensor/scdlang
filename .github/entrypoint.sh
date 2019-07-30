#!/bin/sh
set -e

export PATH="$HOME/.cargo/bin:$PATH"

for cmd in "$@"; do
  echo "Running '$cmd'..."
  if sh -c "$cmd"; then
    [ -z "$BIN" ] && mv target/debug/$BIN $HOME/.cargo/bin/$BIN
    echo
    echo "Successfully ran '$cmd'"
  else
    exit_code=$?
    echo
    echo "Failure running '$cmd', exited with $exit_code"
    exit $exit_code
  fi
done
