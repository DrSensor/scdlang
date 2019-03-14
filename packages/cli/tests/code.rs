mod utils;

#[allow(unused_imports)] // false alarm on rustc 😅
use assert_cmd::prelude::*;
#[allow(unused_imports)] // false alarm on rustc 😅
use assert_fs::{prelude::*, NamedTempFile};
use utils::*;

mod should_ok {
	mod stream_mode {
		use crate::*;
		use predicates::str;

		#[test]
		fn parse_valid_file() {
			let args = Some(format!(
				"{file} --stream",
				file = path::example("simple.scl").unwrap()
			));

			let mut command = subcommand::code(args.as_deref()).unwrap();
			command.assert().success();
		}

		#[test]
		fn save_to_file() {
			let target = NamedTempFile::new("dimple.json").unwrap();
			let args = Some(format!(
				"{file} {dist} --stream",
				file = path::example("simple.scl").unwrap(),
				dist = target.path().display()
			));

			let mut command = subcommand::code(args.as_deref()).unwrap();

			command.assert().success();
			target.assert(path::exists());
			target.assert(path::is_file());
			target.assert(str::is_match(regex::NOEMPTY).unwrap());
		}
	}
}

mod should_fail {
	use crate::*;
	use errcode::*;

	#[test]
	fn parse_invalid_file() {
		let mut command = subcommand::code(Some("🤘.scl --stream")).unwrap();
		command.assert().failure().code(ENOENT);
	}
}
