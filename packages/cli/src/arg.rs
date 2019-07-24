use crate::{error::*, format};
use clap::{Arg, ArgMatches};
use which::which;

pub mod output {
	use super::*;
	use crate::iter::*;

	#[allow(clippy::match_bool)]
	pub fn validate<'s>(args: &ArgMatches) -> Result<(), Error<'s>> {
		use format::{ext::*, XSTATE};
		match (args.value_of(TARGET), args.value_of(FORMAT)) {
			(Some(fmt), Some(out)) => {
				let (target, format) = (fmt.to_string(), out.to_string());
				let error_if_not = |formats: &[&'s str]| match formats.iter().any(|f| f == &out) {
					true => Ok(()),
					false => Err(Error::WrongFormat {
						target,
						format,
						possible_formats: formats.into(),
					}),
				};
				match fmt {
					"xstate" => error_if_not(&XSTATE),
					"smcat" => error_if_not(&SMCAT),
					"graph" => error_if_not(&merge(&[&GRAPH_EASY, &DOT])),
					_ => Ok(()),
				}
			}
			_ => Ok(()),
		}
	}

	pub const DIST: &str = "dist";
	pub fn dist<'o>() -> Arg<'o, 'o> {
		Arg::from_usage("[dist] -o, --output [DIST] 'Output the result to this directory / file'")
			.required_ifs(&format::BLOB.iter().map(|&fmt| (output::FORMAT, fmt)).collect::<Vec<_>>())
		// TODO: submit 👇 as a bug issue to clap-rs (not compatible with args_from_usage)
		// Arg::from_usage("[DIST] 'Output the result to this directory / file'")
		// 	.required_ifs(&format::BLOB.iter().map(|&fmt| (output::FORMAT, fmt)).collect::<Vec<_>>()),
	}

	pub const TARGET: &str = "target";
	pub fn target<'o>() -> Arg<'o, 'o> {
		Arg::from_usage("<target> -f, --format <target> 'Select output format'").possible_values(&{
			let mut possible_values = vec!["xstate", "smcat"];
			if which("graph-easy").is_ok() || which("dot").is_ok() {
				possible_values.push("graph");
			}
			possible_values
		})
	}

	pub const FORMAT: &str = "format";
	pub fn format<'o>() -> Arg<'o, 'o> {
		Arg::from_usage("[format] --as 'Select parser output'")
			.possible_values(&{
				let mut possible_formats = Vec::new();
				possible_formats.merge_from_slice(&format::XSTATE);
				possible_formats.merge_value(format::SMCAT);
				if which("smcat").is_ok() {
					possible_formats.merge_from_slice(&format::ext::SMCAT);
					if which("graph-easy").is_ok() {
						possible_formats.merge_from_slice(&format::ext::GRAPH_EASY);
					}
					if which("dot").is_ok() {
						possible_formats.merge_from_slice(&format::ext::DOT);
					}
				}
				possible_formats
			})
			.default_value_ifs(&[
				(TARGET, Some("xstate"), "json"),
				(TARGET, Some("smcat"), if which("smcat").is_ok() { "smcat" } else { "json" }),
				(TARGET, Some("graph"), "boxart"),
			])
	}

	// TODO: report bug that .requires(FORMAT) not work if .default_value_ifs() in format<'o>() is specify

	pub const EXPORT_NAME: &str = "export";
	pub const EXPORT_NAME_LIST: [&str; 3] = ["typescript", "javascript", "dts"];
	pub fn export_name<'o>() -> Arg<'o, 'o> {
		Arg::from_usage("[export] --name 'Export name'")
			.requires(FORMAT)
			.empty_values(false)
	}
}
