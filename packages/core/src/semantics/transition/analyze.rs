use super::helper::prelude::*;
use crate::{cache, semantics, utils::naming::sanitize};
use semantics::{analyze, Kind, Transition};

impl<'t> analyze::SemanticCheck<'t> for Transition<'t> {
	fn analyze_error(&self, span: Span<'t>, options: &'t Scdlang) -> Result<(), ScdlError> {
		let mut cache_transition = cache::transition()?;

		for transition in self.clone().into_iter() {
			let (current, target) = (sanitize(transition.from.name), sanitize(transition.to.name));

			let t_cache = cache_transition.entry(current).or_default();
			match &transition.at {
				Some(trigger) => {
					if t_cache.contains_key(&None) {
						return Err(options.err_from_span(span, transition.warn_conflict(&t_cache)).into());
					} else if let Some(prev_target) = t_cache.insert(Some(trigger.into()), target) {
						return Err(options.err_from_span(span, transition.warn_duplicate(&prev_target)).into());
					}
				}
				None => {
					if t_cache.keys().any(Option::is_some) {
						return Err(options.err_from_span(span, transition.warn_conflict(&t_cache)).into());
					} else if let Some(prev_target) = t_cache.insert(None, target) {
						return Err(options.err_from_span(span, transition.warn_duplicate(&prev_target)).into());
					}
				}
			}
		}

		Ok(())
	}

	fn into_kinds(self) -> Vec<Kind<'t>> {
		let mut kinds = Vec::new();
		for transition in self.into_iter() {
			kinds.push(transition.into());
		}
		kinds
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