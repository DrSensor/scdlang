use crate::{cli::*, error::Error, print::*, typedef::tuple::Printer};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
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
		let (print, mut machine): Printer<dyn Transpiler> = match args.value_of("format").unwrap() {
			"xstate" => (
				PRINTER("json", Mode::Default),
				match args.value_of("parser").unwrap() {
					"ast" => Box::new(xstate::ast::Machine::default()),
					"asg" => Box::new(xstate::Machine::default()),
					_ => unreachable!(),
				},
			),
			_ => unreachable!(),
		};

		let mut count_parse_err = 0;
		let config = machine.configure();
		config.with_err_path(filepath);

		if args.is_present("stream") {
			let file = File::open(filepath).map_err(Error::IO)?;
			config.auto_clear_cache(false);

			for (i, line) in BufReader::new(file).lines().enumerate() {
				machine.configure().with_err_line(i);
				let expression: String = line.map_err(Error::IO)?;

				if let Err(err) = machine.insert_parse(&expression).map_err(|e| Error::Parse(e.to_string())) {
					Error::report(err, None);
					count_parse_err += 1;
				}
			}

			machine.clear_cache().map_err(Error::Whatever)?;
		} else {
			let file = fs::read_to_string(filepath).map_err(Error::IO)?;
			machine.parse(&file).map_err(|e| Error::Parse(e.to_string()))?;
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, format!("{}", machine)).map_err(Error::IO)?,
			None if atty::isnt(Stream::Stdout) => println!("{}", machine), // non-interactive shell
			None => print.string(machine.to_string()).map_err(|e| Error::Whatever(e.into()))?, // interactive shell
		}

		if count_parse_err > 0 {
			eprintln!();
			Err(Error::Whatever(format!("Found {} error on parsing", count_parse_err).into()))
		} else {
			Ok(())
		}
	}
}
