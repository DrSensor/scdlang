/// TODO: change to parametric test when cargo support it
/// with parameter Ctrl-C, Ctrl-D, Ctrl-Z, `:exit`
mod utils;

#[allow(unused_imports)] // false alarm on rustc 😅
use assert_cmd::prelude::*;
use rexpect::{errors::Error, session::*};
use scrap::*;
use utils::*;

mod should_ok {
	use super::*;

	#[test]
	#[ignore] // wait until https://github.com/murarth/linefeed/issues/56 resolved
	fn non_interactive_mode() -> Result<(), Error> {
		let args = None;
		let mut command = subcommand::eval(args).unwrap();
		command.assert().success();

		let mut repl = spawn_command(command, None)?;
		repl.send_line("A->D")?;
		repl.exp_string(prompt::REPL)?;

		repl.send_control('d')?;
		repl.exp_regex(regex::NOEMPTY)?;
		Ok(())
	}

	#[test]
	#[ignore] // wait until https://github.com/murarth/linefeed/issues/56 resolved
	fn interactive_mode() -> Result<(), Error> {
		let args = Some("--interactive");
		let mut command = subcommand::eval(args).unwrap();
		command.assert().success();

		let mut repl = spawn_command(command, None)?;
		repl.send_line("A->D")?;
		repl.exp_regex(regex::NOEMPTY)?;

		repl.send_line("")?;
		repl.exp_string(prompt::REPL)?;

		repl.send_control('d')?;
		repl.exp_eof()?;
		Ok(())
	}
}
