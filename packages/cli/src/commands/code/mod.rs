use crate::{cli::*, error::Error, print::*, typedef::tuple::Printer};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use scdlang_core::Transpiler;
use scdlang_xstate as xstate;
use std::{
	fs::{self, File},
	io::{BufRead, BufReader},
};

pub struct Code;
impl<'c> CLI<'c> for Code {
	const NAME: &'c str = "code";
	const USAGE: &'c str = "
	<FILE> 'File to print / concatenate'
	[DIST] 'Output the result to this directory / file'
	";

	#[rustfmt::skip]
	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.visible_aliases(&["generate", "gen", "declaration", "declr"])
			.about("Generate from scdlang file declaration to another format")
			.args(&[
				Arg::with_name("stream").help("Parse the file line by line")
					.long("stream"),
				Arg::with_name("format").help("Select output format")
					.long("format").short("f")
					.possible_values(&["xstate"])
					.default_value("xstate"),
				Arg::with_name("parser").help("Select parser engine")
					.hidden(true) // TODO: don't hide it when AST parser is complete (or at least has same feature as ASG)
					.long("parser")
					.possible_values(&["machine",/*TODO: support "typescript"*/])
					.default_value("asg"),
			])
	}

	#[allow(deprecated)]
	fn invoke(args: &ArgMatches) -> Result {
		let filepath = args.value_of("FILE").unwrap();
		let arg_is_stream = args.is_present("stream");
		// TODO: remove ☝️ when cargo-fmt don't expand 👇
		let print_mode = if arg_is_stream { Mode::UseHeader } else { Mode::Default };
		let (print, mut machine): Printer<dyn Transpiler> = match args.value_of("format").unwrap() {
			"xstate" => (
				PRINTER("json", print_mode),
				match args.value_of("parser").unwrap() {
					"machine" => Box::new(xstate::Machine::new()),
					"typescript" => unreachable!("TODO"),
					_ => unreachable!(),
				},
			),
			_ => unreachable!("{} --format {:?}", Self::NAME, args.value_of("format")),
		};

		let mut count_parse_err = 0;
		machine.configure().with_err_path(filepath);

		if args.is_present("stream") {
			let file = File::open(filepath).map_err(Error::IO)?;
			let mut errors = String::new();

			for (i, line) in BufReader::new(file).lines().enumerate() {
				machine.configure().with_err_line(i);
				let expression: String = line.map_err(Error::IO)?;

				if let Err(err) = machine.insert_parse(&expression) {
					errors.push_str(&format!("{}\n\n", err));
					count_parse_err += 1;
				}
			}

			if !errors.is_empty() {
				Error::report(Error::Parse(errors.trim_matches('\n').to_string()), None);
			}
		} else {
			let file = fs::read_to_string(filepath).map_err(Error::IO)?;
			machine.parse(&file).map_err(|e| Error::Parse(e.to_string()))?;
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, format!("{}", machine)).map_err(Error::IO)?,
			//👇if run on non-interactive shell
			None if atty::isnt(Stream::Stdout) => {
				if count_parse_err > 0 {
					println!("Partial Result\n---\n{}\n---", machine)
				} else {
					println!("{}", machine)
				}
			}
			//👇if run on interactive shell
			None => (if args.is_present("stream") {
				print.string_with_header(
					machine.to_string(),
					format!(
						"({fmt}) {title}",
						fmt = args.value_of("format").unwrap(),
						title = (if count_parse_err > 0 { "Partial Result" } else { filepath }).magenta()
					),
				)
			} else {
				print.string(machine.to_string())
			})
			.map_err(|e| Error::Whatever(e.into()))?,
		}

		if count_parse_err > 0 {
			Err(Error::Whatever(format!("Found {} error on parsing", count_parse_err).into()))
		} else {
			Ok(())
		}
	}
}
