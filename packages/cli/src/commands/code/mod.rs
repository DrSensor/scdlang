use crate::{
	cli::{wip::*, Result, CLI},
	error::Error,
};
use clap::{App, Arg, ArgMatches, SubCommand};
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

	fn command<'s>() -> App<'s, 'c> {
		SubCommand::with_name(Self::NAME)
			.aliases(&["generate", "gen", "declaration", "declr"])
			.about("Generate from scdlang file declaration to another format")
			.args(&[Arg::with_name("parser")
				.long("parser")
				.help("Select parser engine")
				.possible_values(&["ast", "asg"])
				.default_value("asg")
				.default_value_if("stream", None, "ast")
				.takes_value(true)])
			.args_from_usage(Self::USAGE)
	}

	fn invoke(args: &ArgMatches) -> Result {
		let machine = match args.value_of("parser").unwrap() {
			"ast" => UNIMPLEMENTED,
			"asg" => UNIMPLEMENTED,
			_ => return Err(Error::Parse("unknown parser engine".to_string())),
		};
		let filepath = args.value_of("FILE").unwrap();

		if args.is_present("stream") {
			let file = match File::open(filepath) {
				Ok(content) => content,
				Err(io_error) => return Err(Error::IO(io_error)),
			};
			for line in BufReader::new(file).lines() {
				let expression: String = line.expect(Self::NAME);
				if let Err(err) = unimplemented_ok() {
					println!("{}", err);
					return Err(Error::Parse(expression));
				}
			}
		} else {
			// unimplemented!("consider adding --stream")
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, format!("{}", machine)).expect(Self::NAME),
			None => println!("{}", machine),
		}

		Ok(())
	}
}
