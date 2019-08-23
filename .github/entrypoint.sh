#!/bin/sh
set -e

mkdir --parents ${HOME}/.bin/
export PATH="$HOME/.cargo/bin:${HOME}/.bin:$PATH"

[ -z $WORKDIR ] || cd $WORKDIR
for cmd in "$@"; do
  echo "Running '$cmd'..."
  if sh -c "$cmd"; then
    echo
    echo "Successfully ran '$cmd'"
  else
    exit_code=$?
    echo
    echo "Failure running '$cmd', exited with $exit_code"
    exit $exit_code
  fi
done
