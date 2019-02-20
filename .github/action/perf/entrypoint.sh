#!/bin/sh
set -e

PERF_HOME=${PERF_HOME:-${HOME}/.perf}
mkdir -p ${PERF_HOME}

cargo --version

${PERF_SETUP:-$(cargo build)}

echo; if [ -z $CARGO_HOME ]; then
  du -sh /usr/local/cargo/registry || true
else
  du -sh $CARGO_HOME/registry || true
fi; echo

sh -c "hyperfine --export-json '${PERF_HOME}/${GITHUB_ACTION}.json' -p '${PERF_PREPARE:-""}' -w 10 $(wrap-args $*)"

${PERF_TEARDOWN}
