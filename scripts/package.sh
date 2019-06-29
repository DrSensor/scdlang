#!/usr/bin/env bash
set -e
shopt -s globstar # TODO: waiting https://github.com/MicrosoftDocs/vsts-docs/issues/4687 to resolved

for cargo_toml in packages/**/Cargo.toml; do
  cargo package --manifest-path $cargo_toml --no-verify $@
done
