#!/bin/sh
set -e

# -------------------- helper -------------------------
register() {
  read json
  git config --global user.name "$(echo ${json} | jq .author.name)"
  git config --global user.email "$(echo ${json} | jq .author.email)"
}

getinfo() {
  read sha
  curl $([ -z $GITHUB_TOKEN ] || echo "-H 'Authorization: token ${GITHUB_TOKEN}'") -s https://api.github.com/repos/${GITHUB_REPOSITORY}/git/commits/${sha} | jq -c .
}
# -----------------------------------------------------

jq -r .after "$GITHUB_EVENT_PATH" | getinfo | register
export PERF_HOME=${PERF_HOME:-${HOME}/.perf}

sh -c "$*"
