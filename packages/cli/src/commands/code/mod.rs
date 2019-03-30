use crate::{cli::*, error::Error, print::*, typedef::tuple::Printer};
use clap::{App, Arg, ArgMatches};
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
		cmd.aliases(&["generate", "gen", "declaration", "declr"])
			.about("Generate from scdlang file declaration to another format")
			.args(&[
				Arg::with_name("stream").help("Parse the file line by line")
					.long("stream")
					.required_if("parser", "ast"),
				Arg::with_name("format").help("Select output format")
					.long("format").short("f")
					.possible_values(&["xstate"])
					.default_value("xstate"),
				Arg::with_name("parser").help("Select parser engine")
					.hidden(true) // TODO: don't hide it when AST parser is complete (or at least has same feature as ASG)
					.long("parser")
					.possible_values(&["ast", "asg"])
					.default_value("asg"),
			])
	}

	fn invoke(args: &ArgMatches) -> Result {
		let filepath = args.value_of("FILE").unwrap();
		let (print, mut machine): Printer<dyn xstate::Parser> = match args.value_of("format").unwrap() {
			"xstate" => (
				PRINTER("json", Mode::Default),
				match args.value_of("parser").unwrap() {
					"ast" => Box::new(xstate::ast::Machine::new()),
					"asg" => Box::new(xstate::Machine::new()),
					_ => unreachable!(),
				},
			),
			_ => unreachable!(),
		};

		if args.is_present("stream") {
			let file = File::open(filepath).map_err(Error::IO)?;
			for line in BufReader::new(file).lines() {
				let expression: String = line.expect(Self::NAME);
				if let Err(err) = machine.insert_parse(&expression) {
					eprintln!("{}", err);
					return Err(Error::Parse(expression));
				}
			}
		} else {
			let file = fs::read_to_string(filepath).map_err(Error::IO)?;
			if let Err(err) = machine.parse(&file) {
				eprintln!("{}", err);
				return Err(Error::Parse(err.to_string()));
			}
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, format!("{}", machine)).map_err(Error::IO)?,
			None => print.string(machine.to_string()).map_err(|e| Error::Whatever(e.into()))?,
		}

		Ok(())
	}
}
