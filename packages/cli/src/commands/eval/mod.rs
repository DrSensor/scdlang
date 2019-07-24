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
				output::export_name().required_ifs(
					output::EXPORT_NAME_LIST
						.iter()
						.map(|&v| ("format", v))
						.collect::<Vec<_>>()
						.as_slice(),
				),
			])
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		let value_of = |arg| args.value_of(arg).unwrap_or_default();
		let (target, output_format) = (value_of(output::TARGET), value_of(output::FORMAT));
		let (export_name, mut repl) = (value_of(output::EXPORT_NAME), Editor::with_config(prompt::CONFIG()) as REPL);

		let mut machine: Box<dyn Transpiler> = match target {
			"xstate" => Box::new({
				let mut machine = xstate::Machine::new();
				let config = machine.configure();
				config.set("output", output_format);
				if output_format.one_of(&output::EXPORT_NAME_LIST) {
					config.set("export_name", &export_name);
				}
				machine
			}),
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
				let smcat = spawn::smcat(if target == "graph" { "dot" } else { output_format })?;
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

		if atty::is(Stream::Stdin) {
			if !args.is_present("interactive") {
				println!("Press Ctrl-D / Ctrl-C to exit and print the final results");
				println!("> type `print` show the result");
			}
			println!("> type `exit` to close this session\n");
		}

		let mut loc = 0;
		let mut last_line = String::new();
		while let Ok(line) = repl.readline(&format!("{} ", prompt::REPL.bold())) {
			machine.configure().with_err_line(loc);
			let line = line.as_str().trim();
			match line {
				"exit" => break,
				"print" if !args.is_present("interactive") => output(machine.to_string(), None)?,
				_ if !line.trim().is_empty() && !line.trim().starts_with("//") => match machine.insert_parse(line) {
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
								line
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
				},
				_ => last_line = line.to_string(),
			};
			loc += 1;
		}

		if !args.is_present("interactive") && last_line == "print" {
			output(machine.to_string(), None)?;
		}

		Ok(())
	}
}

type REPL = Editor<()>;
