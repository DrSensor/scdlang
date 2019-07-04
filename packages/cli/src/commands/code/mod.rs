use crate::{
	arg::output,
	cli::*,
	error::*,
	format,
	iter::*,
	print::*,
	spawn::{self, *},
	Downcast,
};
use atty::Stream;
use clap::{App, Arg, ArgMatches};
use colored::*;
use scdlang::Transpiler;
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
	--stream 'Parse the file line by line'
	";

	fn additional_usage<'s>(cmd: App<'s, 'c>) -> App<'s, 'c> {
		cmd.visible_aliases(&["generate", "gen", "declaration", "declr"])
			.about("Generate from scdlang file declaration to another format")
			.args(&[
				// TODO: submit 👇 as a bug issue to clap-rs (not compatible with args_from_usage)
				// Arg::from_usage("[DIST] 'Output the result to this directory / file'")
				// 	.required_ifs(&format::BLOB.iter().map(|&fmt| (output::FORMAT, fmt)).collect::<Vec<_>>()),
				output::target(),
				output::format(),
			])
	}

	fn invoke(args: &ArgMatches) -> Result<()> {
		let filepath = args.value_of("FILE").unwrap_or_default();
		let target = args.value_of(output::TARGET).unwrap_or_default();
		let output_format = args.value_of(output::FORMAT).unwrap_or_default();
		let mut print = PRINTER(output_format);

		let mut machine: Box<dyn Transpiler> = match target {
			"xstate" => Box::new(match output_format {
				"json" => xstate::Machine::new(),
				"typescript" => unreachable!("TODO: on the next update"),
				_ => unreachable!("{} --as {:?}", Self::NAME, args.value_of(output::FORMAT)),
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
		if which("smcat").is_ok() && target.one_of(&["smcat", "graph"]) {
			use format::ext;
			let smcat = spawn::smcat(output_format)?;
			match target {
				"smcat" => machine = smcat.output_from(machine)?,
				"graph" if which("dot").is_ok() && output_format.one_of(&ext::DOT) => {
					let input = (machine, smcat.downcast()?);
					machine = spawn::dot(output_format).output_from(input)?;
				}
				"graph" if which("graph-easy").is_ok() && output_format.one_of(&ext::GRAPH_EASY) => {
					let input = format::into_legacy_dot(&smcat.output_from(machine)?);
					machine = spawn::graph_easy(output_format)?.output_from(input)?;
				}
				_ => unreachable!("target.one_of(&[\"smcat\", \"graph\"])"),
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
							"({fmt}.{ext}) {title}",
							fmt = target,
							ext = args.value_of(output::FORMAT).unwrap_or_default(),
							title = (if count_parse_err > 0 { "Partial Result" } else { filepath }).magenta()
						),
					)?
				} else {
					print.string(machine)?
				}
			}
		}

		if count_parse_err > 0 {
			Err(Error::Count {
				topic: "parsing",
				count: count_parse_err,
			}
			.into())
		} else {
			Ok(())
		}
	}
}
