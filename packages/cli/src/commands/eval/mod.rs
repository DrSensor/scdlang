use crate::{
	cli::{Result, CLI},
	error::Error,
	print::*,
	prompt,
};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use rustyline::Editor;
use scdlang_xstate::{self as xstate, *};

pub struct Eval;
impl<'c> CLI<'c> for Eval {
	const NAME: &'c str = "eval";
	const USAGE: &'c str = "
	-i --interactive 'Prints result on each expression'
	--strict 'Exit immediately if an error occurred'
	";

	#[rustfmt::skip]
	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.visible_alias("repl")
			.about("Evaluate scdlang expression in interactive manner")
			.args(&[Arg::with_name("format").help("Select output format")
					.long("format").short("f")
					.possible_values(&["xstate"])
					.default_value("xstate")])
	}

	fn invoke(args: &ArgMatches) -> Result {
		let mut repl = Editor::<()>::new();
		let (print, mut machine) = match args.value_of("format").unwrap() {
			"xstate" => (PRINTER("json", Mode::REPL), xstate::Machine::new()),
			_ => unreachable!(),
		};
		let pprint = |string| print.string(string).map_err(|e| Error::Whatever(e.into()));

		let mut parse = |expression: &str| -> Result {
			if !expression.is_empty() {
				match machine.insert_parse(expression) {
					Ok(_) => {
						if args.is_present("interactive") {
							pprint(machine.to_string())?
						}
					}
					Err(err) => {
						println!("{}", err);
						if args.is_present("strict") {
							return Err(Error::Parse(expression.to_string()));
						}
					}
				}
			}
			Ok(())
		};

		if !args.is_present("interactive") && atty::is(Stream::Stdin) {
			println!("Press Ctrl-D to exit and print the final results");
		}

		while let Ok(line) = repl.readline(&format!("{} ", prompt::REPL)) {
			parse(&line)?;
			repl.add_history_entry(line);
		}

		if !args.is_present("interactive") {
			pprint(machine.to_string())?;
		}

		Ok(())
	}
}
