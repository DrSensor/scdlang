#!/bin/sh
set -e

export PERF_HOME=${PERF_HOME:-${HOME}/.perf}

sh -c "$*"
