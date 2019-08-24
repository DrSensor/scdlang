//! Collections of helper module.

pub mod naming;

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

impl<T: Hash> CalculateHash for T {}
pub(crate) trait CalculateHash: Hash {
	fn get_hash(&self) -> u64 {
		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}
}
