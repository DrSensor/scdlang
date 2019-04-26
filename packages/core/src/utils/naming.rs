use std::{fmt, ops};

#[derive(Debug, Clone)]
pub enum Name<'t> {
	Quoted(&'t str),
	Unquoted(&'t str),
}

use Name::*;
impl Name<'_> {
	/// Map unquoted name. Quoted name will be returned as it is
	pub fn map(&self, f: impl FnOnce(&str) -> String) -> String {
		match self {
			Unquoted(name) => f(name),
			Quoted(name) => String::from(*name),
		}
	}

	/// Map quoted name. Unquoted name will be returned as it is
	pub fn map_quoted(&self, f: impl FnOnce(&str) -> String) -> String {
		match self {
			Unquoted(name) => String::from(*name),
			Quoted(name) => f(name),
		}
	}

	/// Map either quoted or unquoted name
	pub fn map_all(&self, f: impl Fn(&str) -> String) -> String {
		match self {
			Quoted(name) | Unquoted(name) => f(name),
		}
	}
}

impl ops::Deref for Name<'_> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		match self {
			Unquoted(name) | Quoted(name) => name,
		}
	}
}

impl PartialEq<str> for Name<'_> {
	fn eq(&self, other: &str) -> bool {
		match self {
			Unquoted(name) | Quoted(name) => name == &other,
		}
	}
}

impl fmt::Display for Name<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Name::Unquoted(name) => write!(f, "{}", name),
			Name::Quoted(name) if name.contains('\n') => write!(f, "`{}`", name),
			Name::Quoted(name) => write!(f, "\"{}\"", name), // double-quote because it compatible with haskell syntax highlighter
		}
	}
}
