#!/bin/sh
set -e

sh -c "hyperfine --prepare 'cargo clean' -w 100 'cargo $*'"
