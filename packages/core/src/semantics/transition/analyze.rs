use super::helper::{prelude::*, transform_key::*};
use crate::{cache, semantics, utils::naming::sanitize, Error};
use semantics::{analyze::*, Kind, Transition};

impl SemanticCheck for Transition<'_> {
	fn check_error(&self) -> Result<Option<String>, Error> {
		// (key, value) = (EventName + guardName, NextState)
		let (mut cache, target) = (cache::transition()?, sanitize(self.to.as_ref().unwrap_or(&self.from).name));
		let t_cache = self.cache_current_state(&mut cache);

		Ok(match &self.at {
			Some(event) => {
				if t_cache.contains_key(&None) && event.name.is_some() {
					Some(self.warn_conflict(&t_cache))
				} else if let Some(prev_target) = t_cache.insert(Some(event.into()), target) {
					Some(self.warn_duplicate(&prev_target))
				} else {
					None
				}
			}
			None => {
				if t_cache.keys().any(EventKey::has_trigger) {
					Some(self.warn_conflict(&t_cache))
				} else if let Some(prev_target) = t_cache.insert(None, target) {
					Some(self.warn_duplicate(&prev_target))
				} else {
					None
				}
			}
		})
	}

	fn check_warning(&self) -> Result<Option<String>, Error> {
		// (key, value) = (EventName + guardName, NextState)
		let (mut cache, target) = (cache::transition()?, sanitize(self.to.as_ref().unwrap_or(&self.from).name));
		let t_cache = self.cache_current_state(&mut cache);

		// WARNING: don't insert anything since the insertion is already done in analyze_error >-down-to-> check_error
		Ok(self.at.as_ref().and_then(|event| {
			let has_same_event = t_cache
				.keys()
				.filter(|&key| key != &Some(event.into()))
				.any(|key| key.get_trigger() == event.name);
			let has_different_target = t_cache.values().any(|key| key != &target);

			if t_cache.keys().any(EventKey::has_guard) && event.guard.is_some() && has_same_event && has_different_target {
				Some(self.warn_nondeterministic(t_cache))
			} else {
				None
			}
		}))
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

	fn analyze_warning(&self, span: Span<'t>, options: &'t Scdlang) -> Result<(), Error> {
		let make_error = |message| options.err_from_span(span, message).into();
		for transition in self.clone().into_iter() {
			if let Some(message) = transition.check_warning()? {
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
type CacheMap = HashMap<Option<String>, String>;
type CachedTransition<'state> = MutexGuard<'state, cache::TransitionMap>;

impl<'t> Transition<'t> {
	fn cache_current_state<'a>(&self, cache: &'t mut CachedTransition<'a>) -> &'t mut CacheMap {
		cache.entry(sanitize(self.from.name)).or_default()
	}
}

impl Transition<'_> {
	fn warn_duplicate(&self, prev_target: &str) -> String {
		match &self.at {
			Some(trigger) => format!(
				"duplicate transition: {} -> {},{} @ {}",
				self.from.name,
				self.to.as_ref().unwrap_or(&self.from).name,
				prev_target,
				trigger
			),
			None => format!(
				"duplicate transient-transition: {} -> {},{}",
				self.from.name,
				self.to.as_ref().unwrap_or(&self.from).name,
				prev_target
			),
		}
	}

	fn warn_conflict(&self, cache: &CacheMap) -> String {
		const REASON: &str = "never had a chance to trigger";
		match &self.at {
			Some(event) => {
				let (prev_target, trigger) = (
					cache.get(&None).expect("cache without trigger"),
					event.name.expect("not auto-transition"),
				);
				format!("{} {} because: {} -> {}", trigger, REASON, self.from.name, prev_target)
			}
			None => {
				let caches = cache.iter().filter(|(event, _)| event.has_trigger());

				let mut messages = format!("conflict with {} {{\n", self.from.name);
				for (event, target) in caches.clone() {
					writeln!(&mut messages, "\t -> {}{}", target, event.as_expression()).expect("utf-8");
				}
				messages += "   }";

				let triggers = caches.filter_map(|(event, _)| event.get_trigger()).collect::<Vec<&str>>();
				write!(&mut messages, "\n   because {} {}", triggers.join(","), REASON).expect("utf-8");
				messages
			}
		}
	}

	fn warn_nondeterministic(&self, cache: &CacheMap) -> String {
		let mut messages = String::from("non-deterministic transition of ");
		match &self.at {
			Some(event) => {
				let guards = cache.keys().filter_map(|k| k.guards_with_same_trigger(event.name));

				writeln!(&mut messages, "{}{} {{", self.from.name, event.name.as_expression()).expect("utf-8");
				for guard in guards {
					let target = cache.get(&event.name.as_key(&guard)).map(String::as_str).unwrap_or("");
					writeln!(&mut messages, "\t-> {} @ [{}]", target, guard).expect("utf-8");
				}
				messages + "   }"
			}
			None => unreachable!("there is no such thing as \"non-deterministic transient transition\""),
		}
	}
}
