#!/bin/sh
set -e

if ! git show-ref --quiet refs/notes/commits; then
  git fetch origin refs/notes/commits:refs/notes/commits
fi

( echo '['; git log --format='{
  refs: "%D"
  subject: %s
  perf: [%N]
  sha: {
    commit: "%H"
    tree: "%T"
    parent: "%P"
  }
  author: {
    name: "%aN"
    email: "%aE"
    date: "%aD"
  }
  commiter: {
    name: "%cN"
    email: "%cE"
    date: "%cD"
  }
}' $*; echo ']' ) | hjson -c | jq .
