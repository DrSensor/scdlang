#![allow(clippy::unit_arg)]
use crate::error::Error;
use atty::Stream;
use prettyprint::{PrettyPrint, PrettyPrintError};

type Closure<'e> = &'e (dyn Fn(String) + 'e);

pub struct Console<'p> {
	pub printer: &'p PrettyPrint,
	pub header: &'p str,
	pub fallback: Result<Closure<'p>, Closure<'p>>,
}

impl Console<'_> {
	pub fn print(self, string: String) -> Result<(), Error> {
		Ok(match self.fallback {
			Ok(fallback_print) if !atty::is(Stream::Stdout) => fallback_print(string),
			Err(fallback_print) if !atty::is(Stream::Stderr) => fallback_print(string),
			_ => self.pretty_print(string).map_err(|e| Error::Whatever(e.into()))?,
		})
	}

	fn pretty_print(self, string: String) -> Result<(), PrettyPrintError> {
		if atty::is(Stream::Stdin) || self.header.is_empty() {
			self.printer.string(string)
		} else {
			self.printer.string_with_header(string, self.header.replace("\n", ""))
		}
	}
}
