use clap::Arg;

pub mod output {
	use super::*;
	use crate::{format, iter::Merge};
	use which::which;

	pub const TARGET: &str = "format";
	pub fn target<'o>() -> Arg<'o, 'o> {
		Arg::with_name(TARGET)
			.help("Select output format")
			.long("format")
			.short("f")
			.possible_values(&["xstate", "smcat"])
			.default_value("xstate")
	}

	pub const FORMAT: &str = "as";
	pub fn format<'o>() -> Arg<'o, 'o> {
		Arg::with_name(FORMAT)
			.help("Select parser output")
			.long("as")
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
			])
	}
}
