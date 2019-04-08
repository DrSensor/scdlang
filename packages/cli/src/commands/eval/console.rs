#![allow(clippy::unit_arg)]
use crate::Error;
use atty::Stream;
use prettyprint::{PrettyPrint, PrettyPrintError};

pub struct Console<'p> {
	pub printer: &'p PrettyPrint,
	pub header: &'p str,
	pub fallback: fn(String),
}

impl Console<'_> {
	pub fn print(self, string: String) -> Result<(), Error> {
		if atty::is(Stream::Stdout) {
			self.pretty_print(string).map_err(|e| Error::Whatever(e.into()))
		} else {
			Ok((self.fallback)(string))
		}
	}

	fn pretty_print(self, string: String) -> Result<(), PrettyPrintError> {
		if atty::is(Stream::Stdin) || self.header.is_empty() {
			self.printer.string(string)
		} else {
			self.printer.string_with_header(string, self.header.replace("\n", ""))
		}
	}
}
