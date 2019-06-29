#!/bin/sh
set -e
shopt -s extglob
# shopt -s globstar # azure-pipelines still use bash 3.1 😞

for cargo_toml in packages/{*,transpiler/*}/Cargo.toml; do
  cargo package --manifest-path $cargo_toml --no-verify $@
done
