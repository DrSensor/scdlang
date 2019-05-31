mod utils;

#[allow(unused_imports)] // false alarm on rustc 😅
use assert_cmd::prelude::*;
#[allow(unused_imports)] // false alarm on rustc 😅
use assert_fs::{prelude::*, NamedTempFile};
use utils::*;

mod should_ok {
	use crate::*;
	use predicates::str;

	static FROM_EXAMPLES: [&str; 2] = ["multi-line.scl", "simple.scl"];
	static FLAGS: &str = "";

	#[test]
	fn parse_valid_file() {
		FROM_EXAMPLES.iter().for_each(|example| parse_file(example, FLAGS))
	}

	#[test]
	fn save_to_file() {
		FROM_EXAMPLES.iter().for_each(|example| save_file(example, FLAGS))
	}

	mod stream_mode {
		use super::*;

		static FROM_EXAMPLE: &str = "simple.scl";
		static FLAGS: &str = "--stream";

		#[test]
		fn parse_valid_file() {
			parse_file(FROM_EXAMPLE, FLAGS)
		}

		#[test]
		fn save_to_file() {
			save_file(FROM_EXAMPLE, FLAGS)
		}
	}

	fn parse_file(input: &str, flags: &str) {
		let flags = normalize(flags);
		let args = Some(format!("{}{}", path::example(input).unwrap(), flags));

		let mut command = subcommand::code(args.as_deref()).unwrap();
		command.assert().success();
	}

	fn save_file(input: &str, flags: &str) {
		let flags = normalize(flags);
		let target = NamedTempFile::new("dimple.json").unwrap();
		let args = Some(format!(
			"{file} {dist}{args}",
			file = path::example(input).unwrap(),
			dist = target.path().display(),
			args = flags
		));

		let mut command = subcommand::code(args.as_deref()).unwrap();

		command.assert().success();
		target.assert(path::exists());
		target.assert(path::is_file());
		target.assert(str::is_match(regex::NOEMPTY).unwrap());
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

fn normalize(flags: &str) -> String {
	if !flags.is_empty() {
		" ".to_owned() + flags
	} else {
		flags.to_string()
	}
}
