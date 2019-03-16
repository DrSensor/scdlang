mod utils;

use crate::{
	cli::{Result, CLI},
	error::Error,
	prompt,
};
use atty::Stream;
use clap::{App, ArgMatches};
use scdlang_xstate::*;
use std::io::{self, prelude::*};
use utils::*;

pub struct Eval;
impl<'c> CLI<'c> for Eval {
	const NAME: &'c str = "eval";
	const USAGE: &'c str = "
	-i --interactive 'Prints result on each expression'
	--strict 'Exit immediately if an error occurred'
	";

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.visible_alias("repl")
			.about("Evaluate scdlang expression in interactive manner")
	}

	fn invoke(args: &ArgMatches) -> Result {
		let stdin = io::stdin();
		let mut machine = ast::Machine::new();
		let prompting = || {
			if atty::is(Stream::Stdin) {
				prompt(prompt::REPL).expect(Self::NAME);
			}
		};

		if atty::is(Stream::Stdin) && !args.is_present("interactive") {
			println!("Press Ctrl-D to exit and print the final results");
		}

		// TODO: change to https://docs.rs/linefeed
		prompting();
		for line in stdin.lock().lines() {
			let expression = line.expect(Self::NAME);
			if !expression.is_empty() {
				if let Err(err) = machine.insert_parse(expression.as_str()) {
					println!("{}", err);
					if args.is_present("strict") {
						return Err(Error::Parse(expression));
					}
				} else if args.is_present("interactive") {
					println!("{}", machine);
				}
			}
			prompting();
		}

		if args.is_present("interactive") {
			print!("\r")
		} else if atty::isnt(Stream::Stdin) {
			println!("{}", machine);
		} else {
			println!("\r{}", machine);
		}

		Ok(())
	}
}
