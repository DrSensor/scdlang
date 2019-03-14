use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Scdlang;

#[cfg(test)]
mod test {
	pub use crate::test;
	pub type Yes = Result<(), String>;

	#[test]
	fn transition_to() -> Yes {
		test::correct_expressions(&[
			r#"A->B"#,
			r#"Alpha-->B"#,
			r#"A--->Beta"#,
			r#"AlphaGo->BetaRust"#,
		])
	}

	mod should_fail_when {
		use super::*;

		#[test]
		fn use_wrong_symbol() -> Yes {
			// From https://github.com/tonsky/FiraCode 😋
			test::wrong_expressions(&[
				// #region transition_to
				r#"A->>B"#, r#"A>->B"#, r#"A>-B"#, r#"A>>-B"#, r#"A~>B"#,
				r#"A~~>B"#,
				// #endregion
			])
		}
	}
}
