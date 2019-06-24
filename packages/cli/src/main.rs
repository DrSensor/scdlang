mod lib;
use lib::*;

use error::Report;

fn main() {
	let matches = cli::build().get_matches();

	if let Err(err) = cli::run(&matches) {
		match matches.subcommand_name() {
			Some(command) => err.report_and_exit(Some(-1), matches.subcommand_matches(command).map(ToOwned::to_owned)),
			None => err.report_and_exit(Some(-1), Some(matches)),
		}
	}
}
