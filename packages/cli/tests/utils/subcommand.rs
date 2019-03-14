use assert_cmd::cargo::*;
use std::{process::Command, result as builtins};

pub type Result = builtins::Result<Command, CargoError>;

fn exec(subcmd: &str, args: Option<&str>) -> Result {
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
	cmd.arg(subcmd);
	if let Some(arguments) = args {
		cmd.args(arguments.split(' '));
	}
	Ok(cmd)
}

#[allow(dead_code)] // false alarm on rustc 😅
pub fn code(args: Option<&str>) -> Result {
	exec("code", args)
}

#[allow(dead_code)] // false alarm on rustc 😅
pub fn eval(args: Option<&str>) -> Result {
	exec("eval", args)
}
