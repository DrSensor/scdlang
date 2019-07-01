use crate::{error::*, format};
use clap::{Arg, ArgMatches};
use which::which;

pub mod output {
	use super::*;
	use crate::iter::Merge;

	pub const TARGET: &str = "target";
	pub fn target<'o>() -> Arg<'o, 'o> {
		Arg::with_name(TARGET)
			.short("f")
			.long("format")
			.help("Select output format")
			.required(true)
			.takes_value(true)
			.possible_values(&{
				let mut possible_values = vec!["xstate", "smcat"];
				if which("graph-easy").is_ok() {
					possible_values.push("graph");
				}
				possible_values
			})
	}

	#[allow(clippy::match_bool)]
	pub fn validate<'s>(args: &ArgMatches) -> Result<(), Error<'s>> {
		use format::{ext::*, XSTATE};
		match (args.value_of(TARGET), args.value_of(FORMAT)) {
			(Some(fmt), Some(out)) => {
				let (target, format) = (fmt.to_string(), out.to_string());
				let error_if_not = |formats: &'s [&'s str]| match formats.iter().any(|f| *f == out) {
					true => Ok(()),
					false => Err(Error::WrongFormat {
						target,
						format,
						possible_formats: formats,
					}),
				};
				match fmt {
					"xstate" => error_if_not(&XSTATE),
					"smcat" => error_if_not(&SMCAT),
					"graph" => error_if_not(&GRAPH_EASY),
					_ => Ok(()),
				}
			}
			_ => Ok(()),
		}
	}

	pub const FORMAT: &str = "format";
	pub fn format<'o>() -> Arg<'o, 'o> {
		Arg::with_name(FORMAT)
			.long("as")
			.help("Select parser output")
			.requires(TARGET)
			.hidden(which("smcat").is_err()) // TODO: don't hide it when support another output (e.g typescript)
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
				(TARGET, Some("xstate"), "json"),
				(TARGET, Some("smcat"), if which("smcat").is_ok() { "smcat" } else { "json" }),
				(TARGET, Some("graph"), "boxart"),
			])
	}
}
