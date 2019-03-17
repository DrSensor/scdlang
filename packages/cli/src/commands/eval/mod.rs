use crate::{
	cli::{Result, CLI},
	error::Error,
	prompt,
};
use atty::Stream;
use clap::{App, ArgMatches};
use linefeed::{Interface, ReadResult};
use scdlang_xstate::*;
use std::io::{self, prelude::*};

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
		let mut machine = ast::Machine::new();
		let repl = Interface::new(env!("CARGO_PKG_NAME")).map_err(Error::IO)?;
		let mut previously_error = false;

		let mut parse = |expression: String| -> Result {
			if !expression.is_empty() {
				match machine.insert_parse(expression.as_str()) {
					Ok(_) => {
						if args.is_present("interactive") {
							println!("{}", machine);
						}
						if previously_error {
							repl.remove_history(repl.history_len() - 1); // remove errored input
							previously_error = false;
						}
					}
					Err(err) => {
						println!("{}", err);
						if args.is_present("strict") {
							return Err(Error::Parse(expression));
						}
						previously_error = true;
					}
				}
				repl.add_history_unique(expression);
			}
			Ok(())
		};

		// parse depend on if it's piped from another process or not
		if !atty::is(Stream::Stdin) {
			for line in io::stdin().lock().lines() {
				parse(line.expect(Self::NAME))?;
			}
		} else {
			println!(
				"Press Ctrl-D to exit {}",
				if !args.is_present("interactive") {
					"and print the final results"
				} else {
					""
				}
			);

			repl.set_prompt(&format!("{} ", prompt::REPL)).map_err(Error::IO)?;
			while let ReadResult::Input(line) = repl.read_line().map_err(Error::IO)? {
				parse(line)?;
			}
		}

		// print final result depend on the condition
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
