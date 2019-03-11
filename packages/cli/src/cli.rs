use crate::error::Error;
use clap::{self, App, AppSettings::*, ArgMatches};

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
	// type Result: Try; // ð use this when trait `Try` become stable

	fn command<'s>() -> App<'s, 'c>;
	fn invoke(args: &ArgMatches) -> Result;
	fn run_on(matches: &ArgMatches) -> Result {
		if let Some(args) = matches.subcommand_matches(Self::NAME) {
			Self::invoke(args)?;
		}
		Ok(())
	}
}

pub struct Main;
impl<'c> CLI<'c> for Main {
	const NAME: &'c str = "Statecharts Rhapsody";
	const USAGE: &'c str = "";

	fn command<'s>() -> App<'s, 'c> {
		App::new(Self::NAME)
			.version(crate_version!())
			.about(crate_description!())
			.settings(&[VersionlessSubcommands, SubcommandRequiredElseHelp])
	}

	fn invoke(_matches: &ArgMatches) -> Result {
		Ok(())
	}
}
