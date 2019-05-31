mod builder;
mod parser;

pub use builder::*;
pub use parser::*;

#[cfg(test)]
mod test {
	pub use crate::test;
	pub type Yes = Result<(), String>;

	#[test]
	fn transition_to() -> Yes {
		test::correct_expressions(&[r#"A->B"#, r#"Alpha-->B"#, r#"A--->Beta"#, r#"AlphaGo->BetaRust"#])
	}

	#[test]
	fn trigger_at() -> Yes {
		test::correct_expressions(&[r#"A->B@C"#, r#"A->B @Carlie"#, r#"A->B @ C"#, r#"A->B@ CarlieErlang"#])
	}

	mod should_fail_when {
		use super::*;

		#[test]
		fn use_wrong_symbol() -> Yes {
			// From https://github.com/tonsky/FiraCode 😋
			test::wrong_expressions(&[
				// #region transition_to
				r#"A->>B"#,
				r#"A>->B"#,
				r#"A>-B"#,
				r#"A>>-B"#,
				r#"A~>B"#,
				r#"A~~>B"#,
				// #endregion
				// #region trigger_at
				r#"A->B@@C"#,
				// #endregion
			])
		}
	}
}
