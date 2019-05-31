#!/bin/sh
set -e

if ! git show-ref --quiet refs/notes/commits; then
  git fetch origin refs/notes/commits:refs/notes/commits
fi

printlog() {
  ( echo '['; git --no-pager log --decorate --first-parent --no-merges --format='{
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
  }' $* --; echo ']' ) | hjson -c
}

if [ -t 1 ]; then
  printlog | jq .
else
  printlog | jq -c .
fi
