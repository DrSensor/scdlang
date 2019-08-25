mod code;
mod eval;

pub use code::*;
pub use eval::*;

use crate::cli::*;
use clap::{crate_description, crate_version, App, AppSettings::*, Arg, ArgMatches, Shell};
use std::{env, io};

pub struct Main;
impl<'c> CLI<'c> for Main {
	const NAME: &'c str = "Statecharts Rhapsody";
	const USAGE: &'c str = "";

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.settings(&[VersionlessSubcommands, ArgRequiredElseHelp])
			.version(crate_version!())
			.about(crate_description!())
			.arg(
				Arg::from_usage("--shell-completion [shell] 'Generate shell completion'").possible_values(&[
					"bash",
					"zsh",
					"fish",
					"elvish",
					"powershell",
				]),
			)
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		if let (Some(shell), Ok(bin_path)) = (args.value_of("shell-completion"), env::current_exe()) {
			build().gen_completions_to(
				bin_path.file_stem().map(|s| s.to_str().expect("utf-8")).expect("appname"),
				match shell {
					"bash" => Shell::Bash,
					"zsh" => Shell::Zsh,
					"fish" => Shell::Fish,
					"elvish" => Shell::Elvish,
					"powershell" => Shell::PowerShell,
					_ => unreachable!("shell {} not supported", shell),
				},
				&mut io::stdout(),
			)
		}
		Ok(())
	}
}
