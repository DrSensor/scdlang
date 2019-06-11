use super::helper::prelude::*;
use crate::{cache, semantics, utils::naming::sanitize};
use semantics::{analyze, Kind, Transition, TransitionType};

impl<'t> analyze::SemanticCheck<'t> for Transition<'t> {
	fn analyze_error(&self, span: Span<'t>, options: &'t Scdlang) -> Result<(), ScdlError> {
		let mut t1_cache = cache::transition()?;

		let (current, target) = (sanitize(self.from.name), sanitize(self.to.name));

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

	fn into_kinds(mut self) -> Vec<Kind<'t>> {
		let mut kinds = Vec::new();
		match self.kind {
			TransitionType::Normal => kinds.push(self.into()),
			TransitionType::Duplex => {
				kinds.push(
					Transition {
						from: self.to.clone(), // arrow::left
						to: self.from.clone(),
						at: self.at.clone(),
						kind: TransitionType::Normal,
					}
					.into(),
				);
				self.kind = TransitionType::Normal;
				kinds.push(self.into()); // arrow::right
			}
			TransitionType::Loop { transient } => {
				/* A ->> B @ C */
				if self.from.name != self.to.name {
					kinds.push(
						Transition /* B -> B @ C */ {
							from: self.to.clone(), // self transition
							to: self.to.clone(),
							at: self.at.clone(),
							kind: TransitionType::Loop { transient },
						}
						.into(),
					);
					if transient {
						self.at = None; // only if /* A >-> B @ C */
					}
					self.kind = TransitionType::Normal;
					kinds.push(self.into());
				} else {
					kinds.push(self.into()); /* ->> B @ C */
				}
			}
			TransitionType::Inside { .. } => unreachable!("TODO: when support StateType::Compound"),
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
