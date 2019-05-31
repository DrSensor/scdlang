mod lib;
pub use lib::*;

use error::Error;

fn main() {
	let matches = cli::build().get_matches();

	if let Err(err) = cli::run(matches) {
		Error::report(err, Some(-1));
	}
}
