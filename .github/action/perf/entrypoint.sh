#!/bin/sh
set -e

cargo --version

sh -c "hyperfine --prepare 'cargo clean' -w 100 $(wrap-args $*)"
