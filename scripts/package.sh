#!/usr/bin/env bash
set -e
shopt -s extglob
# shopt -s globstar # TODO: waiting https://github.com/MicrosoftDocs/vsts-docs/issues/4687 to resolved

for cargo_toml in packages/{*,transpiler/*}/Cargo.toml; do
  cargo package --manifest-path $cargo_toml --no-verify $@
done
