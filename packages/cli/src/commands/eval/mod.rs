mod console;

use crate::{
	cli::{Result, CLI},
	error::Error,
	print::*,
	prompt,
};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use console::*;
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
			"xstate" => (PRINTER("json", print_mode), xstate::Machine::new()),
			_ => unreachable!("{} --format {:?}", Self::NAME, args.value_of("format")),
		};

		#[rustfmt::skip]
		let pprint = |string, header: &str| Console {
			header,
			printer: &print,
			fallback: |s| println!("{}\n", s)
		}.print(string);

		#[rustfmt::skip]
		let epprint = |string, header: &str| Console {
			header: &header.red().to_string(),
			printer: &eprint,
			fallback: |s| eprintln!("{}\n", s)
		}.print(string);

		let mut loc = 0;
		let mut parse = |expression: &str| -> Result {
			machine.configure().with_err_line(loc);
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
				loc += 1;
			}
			Ok(())
		};

		if !args.is_present("interactive") && atty::is(Stream::Stdin) {
			println!("Press Ctrl-D to exit and print the final results");
		}

		while let Ok(line) = repl.readline(&format!("{} ", prompt::REPL.bold())) {
			parse(&line)?;
		}

		if !args.is_present("interactive") {
			pprint(machine.to_string(), "")?;
		}

		Ok(())
	}
}

type REPL = Editor<()>;
