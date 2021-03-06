use crate::{cache, error::Error, external::Builder};
use pest_derive::Parser;

#[derive(Parser, Default, Clone)] // 🤔 is it wise to derive from Copy&Clone ?
#[grammar = "grammar.pest"]
/** __Core__ parser and also [`Builder`].

# Examples
```ignore
use scdlang::semantics::Kind;

let mut parser = Scdlang::new();
parser.with_err_path("test.scl")

for semantic_type in iter_from(source)? {
	match semantic_type {
		Kind::Expression(expr) => {/* expr.[methods] */}
		Kind::Statement(stmnt) => {/* stmnt.[methods] */}
		Kind::Declaration(declr) => {
			/*
			declr.expressions.[methods] // access each expressions
			declr.statements.[methods] //access each statements
			*/
		}
	}
}
```

[`Builder`]: external/trait.Builder.html */
pub struct Scdlang<'g> {
	pub(crate) path: Option<&'g str>,
	pub(crate) line: Option<usize>,

	pub(super) clear_cache: bool, //-|in case for program that need to disable…|
	pub semantic_error: bool,     //-|…then enable semantic error at runtime|
}

impl<'s> Scdlang<'s> {
	/// This method is prefered for instantiating
	/// than using [`Default::default()`](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default)
	pub fn new() -> Self {
		Self {
			clear_cache: true,
			semantic_error: true,
			..Default::default()
		}
	}

	/** > Call this when [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop) cause panic!.

	Manually drop this object which can return Err(Deadlock),
	useful when in _multi-thread process_. */
	pub fn finish(mut self) -> Result<Self, Error> {
		self.clear_cache = false;
		clear_cache()?;
		Ok(self)
	}
}

impl<'g> Builder<'g> for Scdlang<'g> {
	fn with_err_path(&mut self, path: &'g str) -> &mut dyn Builder<'g> {
		self.path = Some(path);
		self
	}

	fn with_err_line(&mut self, line: usize) -> &mut dyn Builder<'g> {
		self.line = Some(line);
		self
	}

	fn with_err_semantic(&mut self, default: bool) -> &mut dyn Builder<'g> {
		self.semantic_error = default;
		self
	}

	fn auto_clear_cache(&mut self, default: bool) -> &mut dyn Builder<'g> {
		self.clear_cache = default;
		self
	}
}

impl<'g> Drop for Scdlang<'g> {
	fn drop(&mut self) {
		if self.clear_cache {
			clear_cache().expect("Deadlock")
		}
	}
}

#[inline]
fn clear_cache() -> Result<(), Error> {
	cache::clear()?.shrink()
}
