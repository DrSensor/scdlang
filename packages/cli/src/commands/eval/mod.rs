mod console;

use crate::{
	arg::output,
	cli::{Result, CLI},
	exec, format,
	print::*,
	prompt,
};
use atty::Stream;
use clap::{App, ArgMatches};
use colored::*;
use console::*;
use rustyline::Editor;
use scdlang_core::Transpiler;
use scdlang_smcat as smcat;
use scdlang_xstate as xstate;
use which::which;

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
			.args(&[output::target(), output::format()])
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		let mut repl: REPL = Editor::with_config(prompt::CONFIG());

		let mut machine: Box<dyn Transpiler> = match args.value_of(output::TARGET).unwrap_or_default() {
			"xstate" => Box::new(xstate::Machine::new()),
			"smcat" => Box::new(smcat::Machine::new()),
			_ => unreachable!("{} --format {:?}", Self::NAME, args.value_of(output::TARGET)),
		};

		let (print_mode, eprint_mode) = if atty::is(Stream::Stdin) || !args.is_present("interactive") {
			(Mode::REPL, Mode::MultiLine)
		} else {
			(Mode::Debug, Mode::Error)
		};
		let print = PRINTER(args.value_of(output::FORMAT).unwrap_or("txt")).change(print_mode);
		let eprint = PRINTER("haskell").change(eprint_mode);

		#[rustfmt::skip]
		let pprint = |string, header: &str| Console {
			header,
			printer: &print,
			fallback: Ok(&|s| println!("{}\n", s))
		}.print(string);

		#[rustfmt::skip]
		let epprint = |string, header: &str| Console {
			header: &header.red().to_string(),
			printer: &eprint,
			fallback: Err(&|s| eprintln!("{}\n", s))
		}.print(string);

		let hook = |input: String| -> Result<String> {
			if which("smcat").is_ok() && args.value_of(output::TARGET).unwrap_or_default() == "smcat" {
				let format = &args.value_of(output::FORMAT).unwrap_or_default();
				let mut result = exec::smcat(format, input)?;

				if which("graph-easy").is_ok() && format::ext::GRAPH_EASY.iter().any(|f| f == format) {
					result = exec::graph_easy(format, result)?;
				}
				Ok(result)
			} else {
				Ok(input)
			}
		};

		let mut loc = 0;
		let mut parse = |expression: &str| -> Result<()> {
			machine.configure().with_err_line(loc);
			if !expression.is_empty() {
				match machine.insert_parse(expression) {
					Ok(_) => {
						if args.is_present("interactive") {
							pprint(hook(machine.to_string())?, expression)?;
						}
					}
					Err(err) => {
						if args.is_present("strict") {
							return Err(err);
						} else {
							epprint(err.to_string(), expression)?;
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
			pprint(hook(machine.to_string())?, "")?;
		}

		Ok(())
	}
}

type REPL = Editor<()>;
