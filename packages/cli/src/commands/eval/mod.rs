use crate::{
	cli::{Result, CLI},
	error::Error,
	print::*,
	prompt,
};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use rustyline::Editor;
use scdlang_xstate::{self as xstate, Transpiler};

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
		let mut repl: REPL = Editor::with_config(prompt::CONFIG());

		let (print_mode, eprint_mode) = if atty::is(Stream::Stdin) || !args.is_present("interactive") {
			(Mode::REPL, Mode::MultiLine)
		} else {
			(Mode::Debug, Mode::Error)
		};

		let eprint = PRINTER("haskell", eprint_mode);
		let (print, mut machine) = match args.value_of("format").unwrap() {
			"xstate" => (PRINTER("json", print_mode), xstate::Machine::default()),
			_ => unreachable!(),
		};

		let pprint = |string, header: &str| {
			(if atty::is(Stream::Stdin) || header.is_empty() {
				print.string(string)
			} else {
				print.string_with_header(string, header.replace("\n", ""))
			})
			.map_err(|e| Error::Whatever(e.into()))
		};

		let epprint = |string, header: &str| {
			(if atty::is(Stream::Stdin) || header.is_empty() {
				eprint.string(string)
			} else {
				eprint.string_with_header(string, format!("{}", header.replace("\n", "").red()))
			})
			.map_err(|e| Error::Whatever(e.into()))
		};

		let mut parse = |expression: &str| -> Result {
			if !expression.is_empty() {
				match machine.insert_parse(expression) {
					Ok(_) => {
						if args.is_present("interactive") {
							pprint(machine.to_string(), expression)?;
						}
					}
					Err(err) => {
						epprint(err.to_string(), expression)?;
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
		}

		if !args.is_present("interactive") {
			pprint(machine.to_string(), "")?;
		}

		Ok(())
	}
}

type REPL = Editor<()>;
