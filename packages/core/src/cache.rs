// Collection of cache variables which help detecting semantics error
// TODO: 🤔 use hot reload https://crates.rs/crates/warmy when import statement is introduced
use crate::error::Error;
use lazy_static::lazy_static;
use std::{
	any::Any,
	collections::HashMap,
	hash::Hash,
	sync::{Mutex, MutexGuard},
};

const NUM_OF_CACHES: usize = 1;
type MapTransition = HashMap<CurrentState, HashMap<Trigger, NextState>>;

// TODO: replace with https://github.com/rust-lang-nursery/lazy-static.rs/issues/111 when resolved
// 🤔 or is there any better way?
// pub static mut TRANSITION: Option<HashMap<Transition, &str>> = None; // doesn't work!
// type LazyMut<T> = Mutex<Option<T>>;
lazy_static! {
	static ref TRANSITION: Mutex<MapTransition> = Mutex::new(HashMap::new());
}

pub fn transition<'a>() -> Result<MutexGuard<'a, MapTransition>, Error> {
	TRANSITION.lock().map_err(|_| Error::Deadlock)
}

/// Clear cache data but preserve the allocated memory for reuse.
/// Call `shrink` to flush out the allocated memory.
/// For example: ```cache::clear()?.shrink()?;```
pub fn clear<'c>() -> Result<Shrink<'c, impl Eq + Hash, impl Any>, Error> {
	let mut cache_list = [
		TRANSITION.lock().map_err(|_| Error::Deadlock)?,
		/*reserved for another caches*/
	];
	cache_list.iter_mut().for_each(|c| c.clear());
	Ok(Shrink(cache_list))
}

pub struct Shrink<'c, K: Eq + Hash, V: Any>(CacheList<'c, K, V>);
impl<K: Eq + Hash, V: Any> Shrink<'_, K, V> {
	/// Shrinks the allocated memory as much as possible
	pub fn shrink(mut self) -> Result<(), Error> {
		self.0.iter_mut().for_each(|c| c.shrink_to_fit());
		Ok(())
	}
}

// TODO: 🤔 consider using this approach http://idubrov.name/rust/2018/06/01/tricking-the-hashmap.html
type CacheList<'c, K, V> = [MutexGuard<'c, HashMap<K, V>>; NUM_OF_CACHES];

type CurrentState = String;
type NextState = String;
type Trigger = Option<String>;
