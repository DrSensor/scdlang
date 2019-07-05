mod console;

use crate::{
	arg::output,
	cli::{Result, CLI},
	format,
	iter::*,
	print::*,
	prompt,
	spawn::{self, *},
	Downcast,
};
use atty::Stream;
use clap::{App, ArgMatches};
use colored::*;
use console::*;
use rustyline::Editor;
use scdlang::Transpiler;
use scdlang_smcat as smcat;
use scdlang_xstate as xstate;
use std::fs;
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
			.args(&[
				output::dist().long_help(
					"The output depend if it's directory or file:
If directory => It will output a numbered file in sequence everytime the REPL produce output.
	Useful if the input is from stdin.
If file => It will be overwriten everytime the REPL produce output, especially if `--interactive` is set.
	Useful if combined with live preview",
				),
				output::target(),
				output::format(),
			])
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		let output_format = args.value_of(output::FORMAT).unwrap_or_default();
		let target = args.value_of(output::TARGET).unwrap_or_default();
		let mut repl: REPL = Editor::with_config(prompt::CONFIG());

		let mut machine: Box<dyn Transpiler> = match target {
			"xstate" => Box::new(xstate::Machine::new()),
			"smcat" | "graph" => {
				let mut machine = Box::new(smcat::Machine::new());
				let config = machine.configure();
				match output_format {
					"ascii" | "boxart" => config.with_err_semantic(true),
					_ => config.with_err_semantic(false),
				};
				machine
			}
			_ => unreachable!("{} --format {:?}", Self::NAME, args.value_of(output::TARGET)),
		};

		let (print_mode, eprint_mode) = if atty::is(Stream::Stdin) || !args.is_present("interactive") {
			(Mode::REPL, Mode::MultiLine)
		} else {
			(Mode::Debug, Mode::Error)
		};
		let print = PRINTER(output_format).change(print_mode);
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

		let hook = |input: String| -> Result<Vec<u8>> {
			use format::ext;
			if which("smcat").is_ok() && target.one_of(&["smcat", "graph"]) {
				let smcat = spawn::smcat(output_format)?;
				let result = match target {
					"smcat" => smcat.output_from(input)?.into(),
					"graph" if which("dot").is_ok() && output_format.one_of(&ext::DOT) => {
						let input = (input, smcat.downcast()?);
						spawn::dot(output_format).output_from(input)?
					}
					"graph" if which("graph-easy").is_ok() && output_format.one_of(&ext::GRAPH_EASY) => {
						let input = format::into_legacy_dot(&smcat.output_from(input)?);
						spawn::graph_easy(output_format)?.output_from(input)?.into()
					}
					_ => unreachable!("--format {}", target),
				};
				Ok(result)
			} else {
				Ok(input.into())
			}
		};

		let output = |input: String, header: Option<String>| -> Result<()> {
			let result = hook(input)?;
			match args.value_of(output::DIST) {
				Some(dist) => fs::write(dist, result)?,
				None => pprint(String::from_utf8(result)?, &header.unwrap_or_default())?,
			};
			Ok(())
		};

		let mut loc = 0;
		let mut parse = |expression: &str| -> Result<()> {
			machine.configure().with_err_line(loc);
			if !expression.trim().is_empty() && !expression.trim().starts_with("//") {
				match machine.insert_parse(expression) {
					Ok(_) => {
						if args.is_present("interactive") {
							// TODO: refactor console.rs
							let header = format!(
								"{} {}",
								if atty::is(Stream::Stdout) {
									format!("{}:", loc + 1).bright_black().to_string()
								} else {
									format!("{}:", loc + 1)
								},
								expression
							);
							output(machine.to_string(), Some(header))?;
						}
					}
					Err(err) => {
						if args.is_present("strict") {
							return Err(err);
						} else {
							epprint(err.to_string(), "")?;
						}
					}
				}
			}
			loc += 1;
			Ok(())
		};

		if !args.is_present("interactive") && atty::is(Stream::Stdin) {
			println!("Press Ctrl-D to exit and print the final results");
		}

		while let Ok(line) = repl.readline(&format!("{} ", prompt::REPL.bold())) {
			match line.as_str() {
				"exit" => break,
				// FIXME: "print" => pprint(hook(machine.to_string())?, "")?,
				_ => parse(&line)?,
			};
		}

		if !args.is_present("interactive") {
			output(machine.to_string(), None)?;
		}

		Ok(())
	}
}

type REPL = Editor<()>;
