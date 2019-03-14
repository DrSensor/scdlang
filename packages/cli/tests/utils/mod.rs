pub mod path;
pub mod subcommand;

#[allow(dead_code)] // false alarm on rustc 😅
pub mod regex {
	pub const NOEMPTY: &str = r".*\S.*";
}

#[allow(dead_code)] // false alarm on rustc 😅
pub mod errcode {
	pub const ENOENT: i32 = 2;
}

//#region remove this when feature(inner_deref) is stable
use std::ops::Deref;

pub trait OptionDeref<T: Deref> {
	fn as_deref(&self) -> Option<&T::Target>;
}

impl<T: Deref> OptionDeref<T> for Option<T> {
	fn as_deref(&self) -> Option<&T::Target> {
		self.as_ref().map(Deref::deref)
	}
}
//#endregion

// TODO: report false alarm issues on rustc repo using this project as an example
