pub mod prompt {
	pub const REPL: &str = ">";
	pub const ERROR: &str = "ERROR:";
}

pub mod wip {
	pub const UNIMPLEMENTED: &str = "not yet implemented";
	pub const fn unimplemented_ok() -> core::result::Result<&'static str, String> {
		Ok(UNIMPLEMENTED)
	}
	pub const fn unimplemented_err() -> core::result::Result<String, &'static str> {
		Err(UNIMPLEMENTED)
	}
}
