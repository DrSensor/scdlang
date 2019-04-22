use super::helper::prelude::*;
use crate::{cache, semantics};
use semantics::{analyze, Kind, Transition};

impl<'t> analyze::SemanticCheck<'t> for Transition<'t> {
	fn analyze_error(&self, span: Span<'t>, options: &'t Scdlang) -> Result<(), ScdlError> {
		let mut t1_cache = cache::transition()?;

		let (current, target) = (self.from.name.to_string(), self.to.name.to_string());
		let t2_cache = t1_cache.entry(current).or_default();

		match &self.at {
			Some(trigger) => {
				if t2_cache.contains_key(&None) {
					return Err(options.err_from_span(span, self.warn_conflict(&t2_cache)).into());
				} else if let Some(prev_target) = t2_cache.insert(Some(trigger.into()), target) {
					return Err(options.err_from_span(span, self.warn_duplicate(&prev_target)).into());
				}
			}
			None => {
				if t2_cache.keys().any(Option::is_some) {
					return Err(options.err_from_span(span, self.warn_conflict(&t2_cache)).into());
				} else if let Some(prev_target) = t2_cache.insert(None, target) {
					return Err(options.err_from_span(span, self.warn_duplicate(&prev_target)).into());
				}
			}
		}

		Ok(())
	}

	fn into_kind(self) -> Kind<'t> {
		Kind::Expression(Box::new(self))
	}
}

use std::collections::HashMap;
impl Transition<'_> {
	fn warn_duplicate(&self, prev_target: &str) -> String {
		match &self.at {
			Some(trigger) => format!(
				"duplicate transition: {} -> {},{} @ {}",
				self.from.name, self.to.name, prev_target, trigger.name
			),
			None => format!(
				"duplicate transient transition: {} -> {},{}",
				self.from.name, self.to.name, prev_target
			),
		}
	}

	fn warn_conflict(&self, cache_target: &HashMap<Option<String>, String>) -> String {
		match &self.at {
			Some(_) => {
				let prev_target = cache_target.get(&None).unwrap();
				format!("conflict with: {} -> {}", self.from.name, prev_target)
			}
			None => {
				let prev_targets: Vec<&str> = cache_target
					.iter()
					.filter_map(|(trigger, target)| trigger.as_ref().and(Some(target.as_str())))
					.collect();
				let prev_triggers: Vec<String> = cache_target.keys().filter_map(ToOwned::to_owned).collect();
				format!(
					"conflict with: {} -> {} @ {}",
					self.from.name,
					prev_targets.join(","),
					prev_triggers.join(",")
				)
			}
		}
	}
}
