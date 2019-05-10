mod code;
mod eval;

pub use code::*;
pub use eval::*;

use crate::cli::*;
use clap::{crate_description, crate_version, App, AppSettings::*, ArgMatches};

pub struct Main;
impl<'c> CLI<'c> for Main {
	const NAME: &'c str = "Statecharts Rhapsody";
	const USAGE: &'c str = "";

	fn command<'s: 'c>() -> App<'c, 's> {
		let cmd = App::new(Self::NAME);
		cmd.settings(&[VersionlessSubcommands, SubcommandRequiredElseHelp])
	}

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.version(crate_version!()).about(crate_description!())
	}

	fn invoke(_matches: &ArgMatches) -> Result {
		Ok(())
	}
}
