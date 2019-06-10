#!/bin/sh
set -e

# -------------------- helper -------------------------
register() {
  read json
  git config --global user.name "$(echo ${json} | jq .author.name)"
  git config --global user.email "$(echo ${json} | jq .author.email)"
}

getinfo() {
  curl $([ -z $GITHUB_TOKEN ] || echo "-H 'Authorization: token ${GITHUB_TOKEN}'") -s https://api.github.com/repos/${GITHUB_REPOSITORY}/git/commits/${GITHUB_SHA} | jq -c .
}
# -----------------------------------------------------

getinfo | register
export PERF_HOME=${PERF_HOME:-${HOME}/.perf}

git fetch -q origin refs/notes/*:refs/notes/*
python --version
pipenv --version

# TODO: support multiple args

# TODO: investigate why it's not working but it works on action/perf
# ${PERF_SETUP:-$(pipenv install --system)}

if [ -z "$PERF_SETUP" ]; then
  pipenv install --system --deploy
  # TODO: support almost all python dependency manager, namely:
  # pip > requirements.txt
  # poetry > pyproject.toml

else
  ${PERF_SETUP}
fi

sh -c "$*"

${PERF_TEARDOWN}
