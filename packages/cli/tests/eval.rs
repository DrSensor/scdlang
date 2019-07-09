/// TODO: change to parametric test when cargo support it
/// with parameter Ctrl-C, Ctrl-D, Ctrl-Z, `:exit`
mod utils;

#[allow(unused_imports)] // false alarm on rustc 😅
use assert_cmd::prelude::*;
use rexpect::{errors::Error, session::*};
use s_crap::*;
use utils::*;
// good references https://thomaslevine.com/computing/shell-testing/

// TODO: deprecate rexpect and use/create others test framework.
// Probably something written in shell/interpreter with TAP output
// Some candidates are: avocado-framework, bats or shpec (using named pipe)
mod should_ok {
	use super::*;
	const TIMEOUT: u64 = 1500; // in milliseconds

	#[test]
	#[ignore]
	fn non_interactive_mode() -> Result<(), Error> {
		let args = Some("--format xstate");
		let mut command = subcommand::eval(args).unwrap();
		command.assert().success();

		let mut repl = spawn_command(command, Some(TIMEOUT))?;
		repl.exp_regex(regex::NOEMPTY)?;
		repl.exp_string(prompt::REPL)?;

		repl.send_line("A->D")?;
		repl.exp_string(prompt::REPL)?;

		repl.send_control('d')?;
		repl.exp_regex(regex::NOEMPTY)?;
		Ok(())
	}

	#[test]
	#[ignore]
	fn interactive_mode() -> Result<(), Error> {
		let args = Some("--format xstate --interactive");
		let mut command = subcommand::eval(args).unwrap();
		command.assert().success();

		let mut repl = spawn_command(command, Some(TIMEOUT))?;
		repl.exp_string(prompt::REPL)?;

		repl.send_line("A->D")?;
		repl.exp_regex(regex::NOEMPTY)?;

		repl.send_line("")?;
		repl.exp_string(prompt::REPL)?;

		repl.send_control('d')?;
		repl.exp_eof()?;
		Ok(())
	}
}
