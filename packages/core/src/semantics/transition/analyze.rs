use super::helper::prelude::*;
use crate::{cache, semantics, utils::naming::sanitize, Error};
use semantics::{analyze::*, Event, Kind, Transition};

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

// WARNING: not performant because of using concatenated String as a key which cause filtering
impl From<&Event<'_>> for String {
	fn from(event: &Event<'_>) -> Self {
		format!("{}?{}", event.name.unwrap_or(""), event.guard.unwrap_or(""))
	}
}

impl<'i> EventKey<'i> for &'i Option<String> {}
trait EventKey<'i>: Into<Option<&'i String>> {
	fn has_trigger(self) -> bool {
		self.into().filter(|e| is_empty(e.rsplit('?'))).is_some()
	}
	fn has_guard(self) -> bool {
		self.into().filter(|e| is_empty(e.split('?'))).is_some()
	}
	fn get_guard(self) -> Option<&'i str> {
		self.into().and_then(|e| none_empty(e.split('?')))
	}
	fn get_trigger(self) -> Option<&'i str> {
		self.into().and_then(|e| none_empty(e.rsplit('?')))
	}
	fn guards_with_same_trigger(self, trigger: Option<&'i str>) -> Option<&'i str> {
		self.into()
			.filter(|e| none_empty(e.rsplit('?')) == trigger)
			.and_then(|e| none_empty(e.split('?')))
	}
	fn triggers_with_same_guard(self, guard: Option<&'i str>) -> Option<&'i str> {
		self.into()
			.filter(|e| none_empty(e.split('?')) == guard)
			.and_then(|e| none_empty(e.rsplit('?')))
	}
	fn as_expression(self) -> String {
		self.into().map(String::as_str).as_expression()
	}
}

impl<'o> Trigger<'o> for &'o Option<&'o str> {}
trait Trigger<'o>: Into<Option<&'o &'o str>> {
	fn as_expression(self) -> String {
		self.into()
			.map(|s| {
				format!(
					" @ {trigger}{guard}",
					trigger = none_empty(s.rsplit('?')).unwrap_or_default(),
					guard = none_empty(s.split('?'))
						.filter(|_| s.contains('?'))
						.map(|g| format!("[{}]", g))
						.unwrap_or_default(),
				)
			})
			.unwrap_or_default()
	}
	fn as_key(self, guard: &str) -> Option<String> {
		Some(format!("{}?{}", self.into().unwrap_or(&""), guard))
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

fn is_empty<'a>(split: impl Iterator<Item = &'a str>) -> bool {
	none_empty(split).is_some()
}

fn none_empty<'a>(split: impl Iterator<Item = &'a str>) -> Option<&'a str> {
	split.last().filter(|s| !s.is_empty())
}

use std::collections::HashMap;
type CacheMap = HashMap<Option<String>, String>;
type CachedTransition<'state> = MutexGuard<'state, cache::MapTransition>;

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
