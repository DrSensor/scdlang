#![allow(unused_imports)]

extern crate clap;
use clap::Shell::*;
use std::{env, path::Path};

#[cfg(not(debug_assertions))]
include!("src/lib.rs");

#[cfg(not(debug_assertions))]
fn main() {
	let mut app = cli::build();
	let ref out_dir = Path::new(env!("PWD")).join("target/release");
	let mut generate_completions = |shell| app.gen_completions(env!("CARGO_PKG_NAME"), shell, out_dir);

	for shell in &[Bash, Fish, Zsh, PowerShell, Elvish] {
		generate_completions(*shell);
	}
}

#[cfg(debug_assertions)]
fn main() {
	// skip if not build as release binary to save development time
}
