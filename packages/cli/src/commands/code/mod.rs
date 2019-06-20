use crate::{cli::*, error::Error, print::*};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use regex::Regex;
use scdlang_core::Transpiler;
use scdlang_smcat as smcat;
use scdlang_xstate as xstate;
use std::{
	fs::{self, File},
	io::{BufRead, BufReader, Write},
	process::{Command, Stdio},
	str,
};
use which::which;

mod format {
	pub const DEFAULT: [&str; 1] = ["json" /*typescript*/];
	pub const SMCAT: [&str; 6] = ["smcat", "dot", "xmi", "svg", "html", "scxml"];
	pub const GRAPH_EASY: [&str; 2] = ["ascii", "boxart"];
}

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
						let mut possible_formats = format::DEFAULT.to_vec();
						if which("smcat").is_ok() {
							possible_formats.append(&mut format::SMCAT.to_vec());
							if which("graph-easy").is_ok() {
								possible_formats.append(&mut format::GRAPH_EASY.to_vec());
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

	fn invoke(args: &ArgMatches) -> Result {
		let filepath = args.value_of("FILE").unwrap();
		let mut print = PRINTER(args.value_of("as").unwrap_or("markdown"));

		let mut machine: Box<dyn Transpiler> = match args.value_of("format").unwrap() {
			"xstate" => Box::new(match args.value_of("as").unwrap() {
				"json" => xstate::Machine::new(),
				"typescript" => unreachable!("TODO: on the next update"),
				_ => unreachable!(),
			}),
			"smcat" => Box::new(smcat::Machine::new()),
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

			print = print.change(Mode::UseHeader);
		} else {
			let file = fs::read_to_string(filepath).map_err(Error::IO)?;
			machine.parse(&file).map_err(|e| Error::Parse(e.to_string()))?;
		}

		let mut machine = machine.to_string();
		if which("smcat").is_ok() && args.value_of("format").unwrap_or_default() == "smcat" {
			let format = &args.value_of("as").unwrap();
			let mut smcat = Command::new("smcat")
				.args(&[
					"--input-type",
					"json",
					"--output-type",
					if format::GRAPH_EASY.iter().any(|f| f == format) {
						"dot"
					} else {
						format
					},
				])
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.spawn()
				.map_err(Error::IO)?;
			write!(smcat.stdin.as_mut().unwrap(), "{}", machine).map_err(Error::IO)?;
			machine = str::from_utf8(&smcat.wait_with_output().map_err(Error::IO)?.stdout)
				.unwrap()
				.to_string();

			if which("graph-easy").is_ok() && format::GRAPH_EASY.iter().any(|f| f == format) {
				let re = Regex::new(r#"( style=["']?\w+["']?)|( penwidth=["']?\d+.\d["']?)"#).unwrap();
				let mut smcat = Command::new("graph-easy")
					.args(&["--as", args.value_of("as").unwrap()])
					.stdin(Stdio::piped())
					.stdout(Stdio::piped())
					.spawn()
					.map_err(Error::IO)?;
				machine = re.replace_all(&machine, "").to_string();
				write!(smcat.stdin.as_mut().unwrap(), "{}", machine).map_err(Error::IO)?;
				machine = str::from_utf8(&smcat.wait_with_output().map_err(Error::IO)?.stdout)
					.unwrap()
					.to_string();
			}
		}

		match args.value_of("DIST") {
			Some(dist) => fs::write(dist, machine).map_err(Error::IO)?,
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
					machine,
					format!(
						"({fmt}) {title}",
						fmt = args.value_of("format").unwrap(),
						title = (if count_parse_err > 0 { "Partial Result" } else { filepath }).magenta()
					),
				)
			} else {
				print.string(machine)
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
