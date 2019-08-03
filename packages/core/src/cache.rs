//! Collection of cache variables which help detecting semantics error.
// TODO: 🤔 use hot reload https://crates.rs/crates/warmy when import statement is introduced
use crate::error::Error;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::*};

// TODO: replace with https://github.com/rust-lang-nursery/lazy-static.rs/issues/111 when resolved
// 🤔 or is there any better way?
// pub static mut TRANSITION: Option<HashMap<Transition, &str>> = None; // doesn't work!
// type LazyMut<T> = Mutex<Option<T>>;
lazy_static! {
	static ref TRANSITION: Mutex<MapTransition> = Mutex::new(HashMap::new());
	static ref WARNING: RwLock<String> = RwLock::new(String::new());
	/*reserved for another caches*/
}

/// Access cached transition safely
pub fn transition<'a>() -> Result<MutexGuard<'a, MapTransition>, Error> {
	TRANSITION.lock().map_err(|_| Error::Deadlock)
}

/// write only caches
pub mod write {
	use super::*;
	/// Access cached warnings safely
	pub fn warning<'a>() -> Result<RwLockWriteGuard<'a, String>, Error> {
		WARNING.write().map_err(|_| Error::Deadlock)
	}
}

/// read only caches
pub mod read {
	use super::*;
	/// Access cached warnings safely
	pub fn warning<'a>() -> Result<RwLockReadGuard<'a, String>, Error> {
		WARNING.read().map_err(|_| Error::Deadlock)
	}
}

/// Clear cache data but preserve the allocated memory for reuse.
pub fn clear() -> Result<Shrink, Error> {
	transition()?.clear();
	write::warning()?.clear();
	/*reserved for another caches*/
	Ok(Shrink)
}

pub struct Shrink;
impl Shrink {
	/// Shrinks the allocated memory as much as possible
	pub fn shrink(self) -> Result<(), Error> {
		transition()?.shrink_to_fit();
		write::warning()?.shrink_to_fit();
		/*reserved for another caches*/
		Ok(())
	}
}

// TODO: 🤔 consider using this approach http://idubrov.name/rust/2018/06/01/tricking-the-hashmap.html
pub(crate) type MapTransition = HashMap<CurrentState, HashMap<Trigger, NextState>>;
pub(crate) type CurrentState = String;
pub(crate) type NextState = String;
pub(crate) type Trigger = Option<String>;
