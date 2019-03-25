use crate::{grammar::Rule, prelude::StateIterator, semantics::Transition};
use pest::iterators::Pairs;
use std::{collections::HashMap, convert::TryFrom};

impl<'p> StateIterator for Pairs<'p, Rule> {
	type Iter = Vec<Self::Item>; // beacuse `FromIterator` not implemented on `Pairs` 😢
	type Map = HashMap<&'p str, Self::Item>;

	fn filter_from(self, state: &str) -> Self::Iter {
		self.filter(filtrate!(|t: Transition| t.from.name == state)).collect()
	}

	fn filter_to(self, state: &str) -> Self::Iter {
		self.filter(filtrate!(|t: Transition| t.to.name == state)).collect()
	}

	fn group_by_from(self, _state: &str) -> Self::Map {
		unimplemented!()
	}

	fn group_by_to(self, _state: &str) -> Self::Map {
		unimplemented!()
	}

	fn dedupe_from(self, _state: &str) -> Self::Iter {
		unimplemented!()
	}

	fn dedupe_to(self, _state: &str) -> Self::Iter {
		unimplemented!()
	}
}

#[cfg(test)]
mod pairs {
	use crate::{prelude::StateIterator, test};

	const FIXTURE: &str = r#"
		A -> B
		C -> B
		A -> C
	"#;

	#[test]
	fn filter_current_state() -> test::parse::Result {
		test::parse::from(FIXTURE, |declaration| {
			let expressions = declaration.filter_from("A");

			assert_eq!(expressions.len(), 2);
			assert!(
				expressions.iter().all(|pair| &pair.as_str()[..4] == "A ->"),
				"all(Transition.from == 'A')"
			);
			Ok(())
		})
	}

	#[test]
	fn filter_next_state() -> test::parse::Result {
		test::parse::from(FIXTURE, |declaration| {
			let expressions = declaration.filter_to("B");

			assert_eq!(expressions.len(), 2);
			assert!(
				expressions.iter().all(|pair| &pair.as_str()[2..6] == "-> B"),
				"all(Transition.to == 'B')"
			);
			Ok(())
		})
	}
}
