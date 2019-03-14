/// TODO: keep track on "Environment variable for Cargo Workspace #3946"
/// https://github.com/rust-lang/cargo/issues/3946

pub use predicates::path::*;
use std::{ffi::OsString, path::Path};

fn get_from(root: &str, dir: &str, filepath: &str) -> Result<String, OsString> {
	Path::new(root)
		.join(dir)
		.join(filepath)
		.into_os_string()
		.into_string()
}

#[allow(dead_code)] // false alarm on rustc 😅
pub fn example(filepath: &str) -> Option<String> {
	get_from(env!("PWD"), "examples", filepath).ok()
}

#[allow(dead_code)]
pub fn template<'p>(filepath: &str) -> Option<String> {
	get_from(env!("PWD"), "templates", filepath).ok()
}
