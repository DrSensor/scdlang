// Collection of cache variables which help detecting semantics error
// TODO: 🤔 use hot reload https://crates.rs/crates/warmy when import statement is introduced
use crate::error::Error;
use lazy_static::lazy_static;
use prelude::*;
use std::sync::{Mutex, MutexGuard};

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

#[macro_use] // TODO: move this declaration to the bottom when rust support that 😂
pub mod prelude {
	pub use std::collections::HashMap;
	#[macro_export]
	macro_rules! drop_map {
		($cache:ident) => {
			let mut map_cache = $cache.lock().map_err(|_| Error::Deadlock)?;
			map_cache.clear();
			map_cache.shrink_to_fit();
		};
	}
}

/// Completely purge cache memory
pub fn drop() -> Result<(), Error> {
	drop_map!(TRANSITION);
	Ok(())
}

// TODO: 🤔 consider using this approach http://idubrov.name/rust/2018/06/01/tricking-the-hashmap.html
type CurrentState = String;
type NextState = String;
type Trigger = Option<String>;
