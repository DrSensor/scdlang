#!/bin/sh
set -e

members=`cargo metadata --no-deps --format-version=1 | jq -r '["-p" + " " + .packages[].name] | join(" ")'`

PERF_HOME=${PERF_HOME:-${HOME}/.perf}
mkdir -p ${PERF_HOME}

cargo --version

${PERF_SETUP:-$(cargo build)}

echo; if [ -z $CARGO_HOME ]; then
  du -sh /usr/local/cargo/registry || true
else
  du -sh $CARGO_HOME/registry || true
fi; echo

sh -c "hyperfine --show-output --export-json '${PERF_HOME}/${GITHUB_ACTION}.json' -p '${PERF_PREPARE:-"cargo clean $members"}' -w 10 $(wrap-args $*)"

${PERF_TEARDOWN}
