#!/bin/sh
set -e
shopt -s extglob
shopt -s globstar

for cargo_toml in packages/**/Cargo.toml; do
  cargo package --manifest-path $cargo_toml --no-verify $@
done
