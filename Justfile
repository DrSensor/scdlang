export MPLBACKEND = "Qt5Agg"

# Start development
start:
	cargo run --quiet --bin scrap

# Run type checker
check:
	cargo check
	mypy scripts

# Run `just +command` whenever some files is changed
@watch +command:
	watchexec just {{command}}

# Run all kind of tests
test: unit

# Autoformat all code
format:
	cargo fmt
	black scripts

# Run linter check on all code
lint:
	cargo clippy
	flake8 scripts

# Clean all artifacts
clean: _clean-analyze
	cargo clean
	pipenv clean

# Run all release build
release:
	cargo build --release

# Run all debug/development build
build:
	cargo build

# Run all unit test
unit:
	cargo test

# Show reports of macro-benchmark
@stats git-flags='':
	./scripts/summary.sh {{git-flags}} | ./scripts/perfsum.py &

# Profile debug/development build
analyze: release _clean-analyze
	heaptrack ./target/release/scrap
	heaptrack --analyze heaptrack.*.zst &
	./scripts/perfquick.sh ./target/release/scrap | jq .

# Install all recommended toolchains
install-toolchains:
	rustup component add rustfmt clippy

@_clean-analyze:
	rm heaptrack.*.zst || true