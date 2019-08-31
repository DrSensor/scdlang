use crate::{arg, commands::*};
use atty::{self, Stream};
use clap::{App, AppSettings::*, ArgMatches, SubCommand};
use std::error;

#[rustfmt::skip]
pub fn build<'a, 'b>() -> App<'a, 'b> {
	Main::command()
		.subcommand(Eval::command())
		.subcommand(Code::command())
}

pub fn run(matches: &ArgMatches) -> Result<()> {
	Main::invoke(&matches)?;
	Eval::run_on(&matches)?;
	Code::run_on(&matches)?;
	Ok(())
}

// TODO: refactor this interface so that there is no need to duplicate [dependecies] into [build-dependencies]
pub trait CLI<'c> {
	const NAME: &'c str;
	const USAGE: &'c str;
	// type Result: Try; // 👈 use this when trait `Try` become stable

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c>;
	fn command<'s: 'c>() -> App<'c, 's> {
		let cmd = SubCommand::with_name(Self::NAME);
		let app = Self::additional_usage(cmd).args_from_usage(Self::USAGE);
		if atty::is(Stream::Stdout) {
			app.setting(ColoredHelp)
		} else {
			app.setting(ColorNever)
		}
	}

	fn invoke(args: &ArgMatches) -> Result<()>;
	fn run_on(matches: &ArgMatches) -> Result<()> {
		if let Some(args) = matches.subcommand_matches(Self::NAME) {
			arg::output::validate(args)?;
			Self::invoke(args)?;
		}
		Ok(())
	}
}

// pub type Result<U> = core::result::Result<U, Box<dyn error::Error>>;
pub type Result<U> = core::result::Result<U, Box<dyn error::Error>>;
