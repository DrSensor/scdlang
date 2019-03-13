use crate::error::Error;
use clap::{App, ArgMatches, SubCommand};

pub mod wip {
	pub const UNIMPLEMENTED: &str = "not yet implemented";
	pub const fn unimplemented_ok() -> core::result::Result<&'static str, String> {
		Ok(UNIMPLEMENTED)
	}
	pub const fn unimplemented_err() -> core::result::Result<String, &'static str> {
		Err(UNIMPLEMENTED)
	}
}

pub type Result = core::result::Result<(), Error>;
pub trait CLI<'c> {
	const NAME: &'c str;
	const USAGE: &'c str;
	// type Result: Try; // 👈 use this when trait `Try` become stable

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c>;
	fn command<'s: 'c>() -> App<'c, 's> {
		let cmd = SubCommand::with_name(Self::NAME);
		Self::additional_usage(cmd).args_from_usage(Self::USAGE)
	}

	fn invoke(args: &ArgMatches) -> Result;
	fn run_on(matches: &ArgMatches) -> Result {
		if let Some(args) = matches.subcommand_matches(Self::NAME) {
			Self::invoke(args)?;
		}
		Ok(())
	}
}
