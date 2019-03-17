use crate::{
	cli::{Result, CLI},
	error::Error,
	wip::*,
};
use clap::{App, Arg, ArgMatches};
use scdlang_xstate::*;
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
	--stream 'Parse the file line by line'
	";

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.aliases(&["generate", "gen", "declaration", "declr"])
			.about("Generate from scdlang file declaration to another format")
			.args(&[Arg::with_name("parser")
				.long("parser")
				.help("Select parser engine")
				.possible_values(&["ast", "asg"])
				.default_value("asg")
				.default_value_if("stream", None, "ast")
				.takes_value(true)])
	}

	fn invoke(args: &ArgMatches) -> Result {
		let filepath = args.value_of("FILE").unwrap();
		let mut machine = match args.value_of("parser").unwrap() {
			"ast" => ast::Machine::new(),
			"asg" => unimplemented!("consider to use `--stream` or `--parser ast`"),
			_ => return Err(Error::Parse("unknown parser engine".to_string())),
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
			let _file = fs::read_to_string(filepath).map_err(Error::IO)?;
			if let Err(err) = unimplemented_ok() {
				eprintln!("{}", err);
				return Err(Error::Parse(err));
			}
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, format!("{}", machine)).expect(Self::NAME),
			None => println!("{}", machine),
		}

		Ok(())
	}
}
