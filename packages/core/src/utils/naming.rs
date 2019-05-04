use std::{fmt, ops};

pub const QUOTES: &[char] = &['\'', '"', '`'];

pub fn sanitize(name: &str) -> String {
	name.trim_matches(QUOTES).replace("\\", "")
}

#[derive(Debug, Clone)]
pub enum Name<'t> {
	Quoted(String),
	Unquoted(&'t str),
}

use Name::*;
impl<'t> From<&'t str> for Name<'t> {
	fn from(name: &'t str) -> Self {
		let first_char = name.chars().next().unwrap_or_default();
		let last_char = name.chars().last().unwrap_or_default();

		if first_char == last_char && QUOTES.contains(&last_char) {
			Quoted(sanitize(name))
		} else {
			Unquoted(name)
		}
	}
}

impl Name<'_> {
	/// Map unquoted name. Quoted name will be returned as it is
	pub fn map(&self, f: impl FnOnce(&str) -> String) -> String {
		match self {
			Unquoted(name) => f(name),
			Quoted(name) => name.to_owned(),
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
			Quoted(name) => f(name),
			Unquoted(name) => f(name),
		}
	}
}

impl ops::Deref for Name<'_> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		match self {
			Quoted(name) => name,
			Unquoted(name) => name,
		}
	}
}

impl PartialEq<str> for Name<'_> {
	fn eq(&self, other: &str) -> bool {
		match self {
			Quoted(name) => name == other,
			Unquoted(name) => name == &other,
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
