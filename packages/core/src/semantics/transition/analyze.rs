use super::helper::prelude::*;
use crate::{cache, semantics, utils::naming::sanitize, Error};
use semantics::{analyze::*, Kind, Transition};

impl SemanticCheck for Transition<'_> {
	fn check_error(&self) -> Result<Option<String>, Error> {
		let mut cache_transition = cache::transition()?;

		let (current, target) = (sanitize(self.from.name), sanitize(self.to.name));
		let t_cache = cache_transition.entry(current).or_default();

		Ok(match &self.at {
			Some(trigger) => {
				if t_cache.contains_key(&None) {
					Some(self.warn_conflict(&t_cache))
				} else if let Some(prev_target) = t_cache.insert(Some(trigger.into()), target) {
					Some(self.warn_duplicate(&prev_target))
				} else {
					None
				}
			}
			None => {
				if t_cache.keys().any(Option::is_some) {
					Some(self.warn_conflict(&t_cache))
				} else if let Some(prev_target) = t_cache.insert(None, target) {
					Some(self.warn_duplicate(&prev_target))
				} else {
					None
				}
			}
		})
	}
}

impl<'t> SemanticAnalyze<'t> for Transition<'t> {
	fn analyze_error(&self, span: Span<'t>, options: &'t Scdlang) -> Result<(), Error> {
		let make_error = |message| options.err_from_span(span, message).into();
		for transition in self.clone().into_iter() {
			if let Some(message) = transition.check_error()? {
				return Err(make_error(message));
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
