#!/bin/sh
set -e

cargo --version

${PERF_SETUP:-$(cargo build)}

echo; if [ -z $CARGO_HOME ]; then
  du -sh /usr/local/cargo/registry || true
else
  du -sh $CARGO_HOME/registry || true
fi; echo

sh -c "hyperfine --prepare '${PERF_PREPARE:-""}' -w 10 $(wrap-args $*)"

${PERF_TEARDOWN}
