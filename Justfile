export MPLBACKEND="Qt5Agg"
export RUST_BACKTRACE="1"

# Start development
start: clear
	cargo run --quiet --bin scrap

# Run type checker
check: clear
	cargo check
	mypy scripts

# Run `just +command` whenever some files is changed
@watch +command:
	watchexec --restart --clear just {{command}}

# Run all kind of tests
test: unit

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
	./scripts/summary.sh {{git-flags}} | ./scripts/perfsum.py

# Profile debug/development build
analyze: release _clean-analyze
	heaptrack ./target/release/scrap
	heaptrack --analyze heaptrack.*.zst &
	./scripts/perfquick.sh ./target/release/scrap | jq .

# Install all dependencies
install: install-toolchains
	cargo build --all
	pipenv install --dev

# Install all recommended toolchains
install-toolchains:
	rustup component add rustfmt clippy
	cargo install hjson
# pipenv lock --requirements --dev | pipenv install --dev --requirements -

@_clean-analyze:
	rm --force heaptrack.*.zst || true