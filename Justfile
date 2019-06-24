export MPLBACKEND := "Qt5Agg"

# Start development
start: clear
	cargo run --quiet --bin scrap

# Run type checker
check: clear
	cargo check
	mypy scripts

# Run with backtrace on
@trace mode +command:
	RUST_BACKTRACE={{mode}} just {{command}}

# Run `just +command` whenever some files is changed
@watch command +args='':
	watchexec --restart --clear 'just {{command}} {{args}}'

# Run all kind of tests
test: unit integration

# Generate and open the documentation
docs +args='':
	cargo doc --no-deps {{args}}

# Autoformat all code
format:
	cargo fmt
	black scripts

# Run linter check on all code
lint: clear
	cargo clippy --tests
	flake8 scripts

# Remove all artifacts but not with the dependencies
@clear: _clean-analyze
	cargo clean $(cargo metadata --no-deps --format-version=1 | jq -r '["-p" + " " + .packages[].name] | join(" ")')

# Remove all artifacts including the dependencies
clean: _clean-analyze
	cargo clean
	pipenv clean

version_subjects := "Cargo.lock Dockerfile 'packages/**/Cargo.toml'"
# Prepare for release
release version:
	#!/usr/bin/env bash
	./scripts/version.py {{version}} && cargo check
	git add --ignore-removal {{version_subjects}}
	TAG=`(git describe --abbrev=0 || echo 0.0.0) 2>/dev/null | ./scripts/version.py {{version}}`
	git commit -S --edit --message "Release v${TAG}" \
	&& git tag --annotate $TAG --message "$(git log -1 --pretty=%B)" --sign
	if [ $? -ne 0 ]; then
		git reset {{version_subjects}}
		./scripts/version.py {{version}}- && cargo check
	fi

# Run all debug/development build
build args='':
	cargo build {{args}}

# Run all unit test
unit:
	cargo test --lib --all --exclude scrap -- --test-threads=1

# Run all integration test
integration:
	cargo test --tests -p scrap -- --test-threads=1

# Show reports of macro-benchmark
@stats git-flags='':
	./scripts/summary.sh {{git-flags}} | ./scripts/perfsum.py

# Profile debug/development build
analyze +args: _clean-analyze
	just build release
	heaptrack ./target/release/scrap {{args}}
	heaptrack --analyze heaptrack.*.zst &
	./scripts/perfquick.sh './target/release/scrap {{args}}' | jq .

# Install all dependencies
install: install-toolchains
	cargo build --all
	pipenv install --dev

# Install all recommended toolchains
install-toolchains:
	rustup component add rls rustfmt clippy rust-src rust-analysis
	# pip install hooks4git hjson --user
# pipenv lock --requirements --dev | pipenv install --dev --requirements -

@_clean-analyze:
	rm --force heaptrack.*.zst || true