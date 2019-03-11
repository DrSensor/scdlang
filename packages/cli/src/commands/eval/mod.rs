mod utils;

use crate::{
	cli::{wip::*, Result, CLI},
	error::Error,
};
use atty::Stream;
use clap::{App, ArgMatches, SubCommand};
use std::io::{self, prelude::*};
use utils::*;

pub struct Eval;
impl<'c> CLI<'c> for Eval {
	const NAME: &'c str = "eval";
	const USAGE: &'c str = "
	-i --interactive 'Prints result on each expression'
	--strict 'Exit immediately if an error occurred'
	";

	fn command<'s>() -> App<'s, 'c> {
		SubCommand::with_name(Self::NAME)
			.visible_alias("repl")
			.about("Evaluate scdlang expression in interactive manner")
			.args_from_usage(Self::USAGE)
	}

	fn invoke(args: &ArgMatches) -> Result {
		let stdin = io::stdin();
		let machine = UNIMPLEMENTED;
		let prompting = || {
			if atty::is(Stream::Stdin) {
				prompt("> ").expect(Self::NAME);
			}
		};

		if atty::is(Stream::Stdin) && !args.is_present("interactive") {
			println!("Press Ctrl-D to exit and print the final results");
		}

		prompting();
		for line in stdin.lock().lines() {
			let expression = line.expect(Self::NAME);
			if let Err(err) = unimplemented_ok() {
				println!("{}", err);
				if args.is_present("strict") {
					return Err(Error::Parse(expression));
				}
			} else if args.is_present("interactive") {
				println!("{}", machine);
			}
			prompting();
		}

		if !args.is_present("interactive") {
			if atty::isnt(Stream::Stdin) {
				println!("{}", machine);
			} else {
				println!("\r{}", machine);
			}
		} else {
			print!("\r")
		}

		Ok(())
	}
}
