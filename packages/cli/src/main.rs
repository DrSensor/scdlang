#[macro_use]
extern crate clap;

pub mod cli;
mod commands;
pub mod error;

use cli::CLI;
use commands::*;
use error::Error;

fn main() {
	let matches = Main::command()
		.subcommand(Eval::command())
		.subcommand(Code::command())
		.get_matches();

	let run = || -> Result<(), Error> {
		Main::run_on(&matches)?;
		Eval::run_on(&matches)?;
		Code::run_on(&matches)?;
		Ok(())
	};

	if let Err(err) = run() {
		Error::report(err, Some(-1));
	}
}

mod lib;
pub use lib::*;
