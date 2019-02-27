export MPLBACKEND = "Qt5Agg"

start: format
	cargo run --quiet --bin scrap

watch +command:
	watchexec just {{command}}

test: unit

format:
	cargo fmt

lint: format
	cargo clippy

clean: format
	cargo clean

# Run all release build
release: format
	cargo build --release

# Run all unit test
unit: format
	cargo test

# Show reports of macro-benchmark
perfsum git-flags='':
	./scripts/summary.sh {{git-flags}} | ./scripts/perfsum.py

install-toolchains:
	rustup add component rustfmt clippy