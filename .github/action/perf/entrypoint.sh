#!/bin/sh
set -e

cargo --version
cargo build

if [ -z $CARGO_HOME ]; then
  du -sh /usr/local/cargo/registry || true
else
  du -sh $CARGO_HOME/registry || true
fi

sh -c "hyperfine --prepare 'cargo clean' -w 10 $(wrap-args $*)"
