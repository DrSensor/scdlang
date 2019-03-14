mod utils;

#[allow(unused_imports)] // false alarm on rustc 😅
use assert_cmd::prelude::*;
use utils::*;
use scrap::*;

mod should_ok {
	use super::*;
	use assert_fs::{
		prelude::*,
		NamedTempFile,
	};

	#[test]
	fn parse_valid_file() {
		let args = path::example("simple.scl");
		let mut command = subcommand::code(args.as_deref()).unwrap();
		command.assert().success();
	}

	#[test]
	fn save_to_file() {
		let file = path::example("simple.scl").unwrap();
		let dist = NamedTempFile::new("dimple.scl").unwrap();

		let args = Some(format!("{} {}", file, dist.path().display()));
		let mut command = subcommand::code(args.as_deref()).unwrap();

		command.assert().success();
		dist.assert(path::exists());
		dist.assert(wip::UNIMPLEMENTED);
	}
}

mod should_fail {
	use super::*;

	#[test]
	fn parse_invalid_file() {
		let mut command = subcommand::code(Some("🤘.scl")).unwrap();
		command.assert().failure().code(2); // ENOENT
	}
}
