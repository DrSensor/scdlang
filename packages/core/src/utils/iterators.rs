// `type Associated: IntoIterator;` is there because `impl Iterator` is not allowed inside trait [E0562]

pub trait StateIterator: Iterator {
	type Iter: IntoIterator;
	type Map: IntoIterator;

	/// Filter by the current state
	fn filter_from(self, state: &str) -> Self::Iter;

	/// Filter by the target/next state
	fn filter_to(self, state: &str) -> Self::Iter;

	/// Group by the current state
	fn group_by_from(self, state: &str) -> Self::Map;

	/// Group by the target/next state
	fn group_by_to(self, state: &str) -> Self::Map;

	/// Remove duplicated current state
	fn dedupe_from(self, state: &str) -> Self::Iter;

	/// Remove duplicated target/next state
	fn dedupe_to(self, state: &str) -> Self::Iter;
}
