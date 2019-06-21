use crate::{cli::*, error::*, exec, format, prelude::*, print::*};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use scdlang_core::Transpiler;
use scdlang_smcat as smcat;
use scdlang_xstate as xstate;
use std::{
	fs::{self, File},
	io::{BufRead, BufReader},
	str,
};
use which::which;

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
					.possible_values(&["xstate", "smcat"])
					.default_value("xstate"),
				Arg::with_name("as").help("Select parser output")
					.hidden(which("smcat").is_err()) // TODO: don't hide it when support another output (e.g typescript)
					.long("as").requires("format")
					.possible_values(&{
						let mut possible_formats = Vec::new();
						possible_formats.merge_from_slice(&format::XSTATE);
						possible_formats.merge_value(format::SMCAT);
						if which("smcat").is_ok() {
							possible_formats.merge_from_slice(&format::ext::SMCAT);
							if which("graph-easy").is_ok() {
								possible_formats.merge_from_slice(&format::ext::GRAPH_EASY);
							}
						}
						possible_formats
					})
					.default_value_ifs(&[
						("format", Some("xstate"), "json"),
						("format", Some("smcat"), if which("smcat").is_ok() { "smcat" } else { "json" })
					]),
			])
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		let filepath = args.value_of("FILE").unwrap_or_default();
		let mut print = PRINTER(args.value_of("as").unwrap_or("txt"));

		let mut machine: Box<dyn Transpiler> = match args.value_of("format").unwrap_or_default() {
			"xstate" => Box::new(match args.value_of("as").unwrap_or_default() {
				"json" => xstate::Machine::new(),
				"typescript" => unreachable!("TODO: on the next update"),
				_ => unreachable!("{} --as {:?}", Self::NAME, args.value_of("as")),
			}),
			"smcat" => Box::new(smcat::Machine::new()),
			_ => unreachable!("{} --format {:?}", Self::NAME, args.value_of("format")),
		};

		let mut count_parse_err = 0;
		machine.configure().with_err_path(filepath);

		if args.is_present("stream") {
			let file = File::open(filepath)?;
			let mut errors = String::new();

			for (i, line) in BufReader::new(file).lines().enumerate() {
				machine.configure().with_err_line(i);
				let expression: String = line?;

				if let Err(err) = machine.insert_parse(&expression) {
					errors.push_str(&format!("{}\n\n", err));
					count_parse_err += 1;
				}
			}

			if !errors.is_empty() {
				Error::StreamParse(errors.trim_matches('\n')).report();
			}

			print = print.change(Mode::UseHeader);
		} else {
			let file = fs::read_to_string(filepath)?;
			machine.parse(&file)?;
		}

		let mut machine = machine.to_string();
		if which("smcat").is_ok() && args.value_of("format").unwrap_or_default() == "smcat" {
			let format = &args.value_of("as").unwrap_or_default();
			machine = exec::smcat(format, machine)?;

			if which("graph-easy").is_ok() && format::ext::GRAPH_EASY.iter().any(|f| f == format) {
				machine = exec::graph_easy(format, machine)?;
			}
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, machine)?,
			//👇if run on non-interactive shell
			None if atty::isnt(Stream::Stdout) => {
				if count_parse_err > 0 {
					println!("Partial Result\n---\n{}\n---", machine)
				} else {
					println!("{}", machine)
				}
			}
			//👇if run on interactive shell
			None => {
				if args.is_present("stream") {
					print.string_with_header(
						machine,
						format!(
							"({fmt}) {title}",
							fmt = args.value_of("format").unwrap(),
							title = (if count_parse_err > 0 { "Partial Result" } else { filepath }).magenta()
						),
					)?
				} else {
					print.string(machine)?
				}
			}
		}

		if count_parse_err > 0 {
			Err(Error::CountError {
				topic: "parsing",
				count: count_parse_err,
			}
			.into())
		} else {
			Ok(())
		}
	}
}
