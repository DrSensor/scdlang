use super::{util::Offset, PestError};
use crate::{grammar::*, Scdlang};
use pest::error::ErrorVariant::{CustomError, ParsingError};
use std::error;

impl FineTune for PestError {
	fn tune_variant(mut self) -> Self {
		use Rule::*;
		if let ParsingError { positives, negatives } = self.variant {
			let exclude_positives = [EOI];
			let positives = match &positives[..] {
				[EOI, Symbol::at, Name::state] => vec![Symbol::at],
				_ => positives
					.into_iter()
					.filter(|rule| exclude_positives.iter().all(|r| r != rule))
					.collect(),
			};

			let exclude_negatives = [EOI, PASCAL_CASE, QUOTED];
			let negatives = negatives
				.into_iter()
				.filter(|rule| exclude_negatives.iter().all(|r| r != rule))
				.collect::<Vec<Rule>>();

			if positives.is_empty() && negatives.is_empty() {
				self.variant = CustomError {
					message: "invalid syntax".to_string(),
				}
			} else {
				self.variant = ParsingError { positives, negatives }
			}
		}
		self
	}
}

impl<'t> Scdlang<'t> {
	pub(crate) fn reformat_error(&self, source: &str, mut error: PestError) -> PestError {
		error = error.tune_variant();
		if let Some(offset) = self.line {
			error = error.with_offset(offset, source);
		}
		if let Some(path) = self.path {
			error = error.with_path(path);
		}
		error.renamed_rules(|rule| match rule {
			Symbol::arrow::right => "->".to_string(),
			Symbol::arrow::left => "<-".to_string(),
			Symbol::at => "@".to_string(),
			Rule::transition => "-> or <-".to_string(),
			_ => format!("{:?}", rule),
		})
	}
}

/// Fine tune syntax error
pub(super) trait FineTune: error::Error {
	fn tune_variant(self) -> Self;
}
