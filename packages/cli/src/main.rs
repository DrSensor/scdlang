mod lib;
use lib::*;

use error::Report;

fn main() {
	let matches = cli::build().get_matches();

	if let Err(err) = cli::run(&matches) {
		err.report_and_exit(Some(-1), Some(matches));
	}
}
