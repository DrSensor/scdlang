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
		test::correct_expressions(&[r#"A->B"#, r#"Alpha -->B"#, r#"A ---> Beta"#, r#"AlphaGo->BetaRust"#])
	}

	#[test]
	fn transition_from() -> Yes {
		test::correct_expressions(&[r#"A<-B"#, r#"Alpha<-- B"#, r#"A <--- Beta"#, r#"AlphaGo<-BetaRust"#])
	}

	#[test]
	fn toggle_transition() -> Yes {
		test::correct_expressions(&[
			r#"A<->B@C"#,
			r#"Alpha<-->B"#,
			r#"A<--->Beta"#,
			r#"Alpha <-> B @Carl"#,
			r#"AlphaGo<->BetaRust @ CarlErlang"#,
		])
	}

	#[test]
	fn trigger_at() -> Yes {
		test::correct_expressions(&[r#"A->B@C"#, r#"A->B @Carl"#, r#"A->B @ C"#, r#"A->B@ CarlErlang"#])
	}

	#[test]
	fn loop_to() -> Yes {
		test::correct_expressions(&[r#"A->>B@C"#, r#"A -->>B @Carl"#, r#"A --->> B @ C"#, r#"A->>B@ CarlErlang"#])
	}

	#[test]
	fn loop_from() -> Yes {
		test::correct_expressions(&[r#"A<<-B@C"#, r#"A<<-- B @Carl"#, r#"A <<--- B @ C"#, r#"A<<-B@ CarlErlang"#])
	}

	#[test]
	fn transient_loop_to() -> Yes {
		test::correct_expressions(&[r#"A>->B@C"#, r#"A >-->B @Carl"#, r#"A >---> B @ C"#, r#"A>->B@ CarlErlang"#])
	}

	#[test]
	fn transient_loop_from() -> Yes {
		test::correct_expressions(&[r#"A<-<B@C"#, r#"A<--< B @Carl"#, r#"A <---< B @ C"#, r#"A<-<B@ CarlErlang"#])
	}

	#[test]
	fn self_transition() -> Yes {
		test::correct_expressions(&[r#"->>B"#, r#"->>B @ C"#, r#">-> B"#, r#">-> B @C"#])
	}

	mod should_fail_when {
		use super::*;

		#[test]
		fn use_wrong_symbol() -> Yes {
			// From https://github.com/tonsky/FiraCode 😋
			test::wrong_expressions(&[
				// #region transition_to
				r#"A>-B"#,
				r#"A>>-B"#,
				r#"A~>B"#,
				r#"A~~>B"#,
				// #endregion
				// #region transition_from
				r#"A-<B"#,
				r#"A-<<B"#,
				r#"A<~B"#,
				r#"A<~~B"#,
				// #endregion
				// #region trigger_at
				r#"A<-B@@C"#,
				// #endregion
				// #region toggle_transition
				r#"A>-<B"#,
				r#"A>>-<<B"#,
				r#"A<~>B"#,
				r#"A<~~>B"#,
				// #endregion
				// #region self_transition
				r#"-<B"#,
				r#">-B"#,
				r#"-<<B"#,
				r#">>-B"#,
				r#"~>B"#,
				r#"~~>B"#,
				r#"<<-B"#,
				r#"B<<-"#,
				r#"<-<B"#,
				r#"B<-<"#,
				// #endregion
			])
		}
	}
}
