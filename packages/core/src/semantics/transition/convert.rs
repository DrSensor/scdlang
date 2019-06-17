#![allow(deprecated)]
use super::helper::{get, prelude::*};
use crate::semantics::{Event, StateType, Transition, TransitionType};
use std::convert::TryFrom;

impl<'t> TryFrom<TokenPair<'t>> for Transition<'t> {
	type Error = ScdlError;

	fn try_from(pair: TokenPair<'t>) -> Result<Self, Self::Error> {
		use ScdlError::*;
		let rule = pair.as_rule();

		let mut lhs = "";
		let mut ops = Rule::EOI;
		let mut rhs = "";
		let mut event = None;

		if let Rule::expression = rule {
			// determine the lhs, rhs, and operators
			for span in pair.into_inner() {
				match span.as_rule() {
					Rule::StateName => lhs = span.as_str(),
					Rule::transition => {
						// TODO: waiting for https://github.com/rust-lang/rfcs/pull/2649 (Destructuring without `let`)
						let (operators, target) = get::transition(span);
						rhs = target;
						ops = operators;
					}
					Rule::self_transition => {
						let (operators, target) = get::transition(span);
						rhs = target;
						lhs = rhs;
						ops = operators;
					}
					Rule::trigger => {
						event = Some(Event {
							name: get::trigger(span),
						})
					}
					_ => unreachable!("Rule::{:?}", span.as_rule()),
				}
			}

			// determine the current, next, and type of the State
			let (transition_type, (current_state, next_state)) = match ops {
				Symbol::double_arrow::right => (
					TransitionType::Loop { transient: false },
					get::state(lhs, rhs, &StateType::Atomic),
				),
				Symbol::tail_arrow::right => (
					TransitionType::Loop { transient: true },
					get::state(lhs, rhs, &StateType::Atomic),
				),
				Symbol::arrow::right => (TransitionType::Normal, get::state(lhs, rhs, &StateType::Atomic)),
				Symbol::arrow::both => (TransitionType::Toggle, get::state(lhs, rhs, &StateType::Atomic)),
				Symbol::arrow::left => (TransitionType::Normal, get::state(rhs, lhs, &StateType::Atomic)),
				Symbol::tail_arrow::left => (
					TransitionType::Loop { transient: true },
					get::state(rhs, lhs, &StateType::Atomic),
				),
				Symbol::double_arrow::left => (
					TransitionType::Loop { transient: false },
					get::state(rhs, lhs, &StateType::Atomic),
				),
				_ => unreachable!("Rule::{:?}", &ops),
			};

			// register into Transition graph
			Ok(Transition {
				from: current_state,
				to: next_state,
				at: event,
				kind: transition_type,
			})
		} else {
			Err(WrongRule(rule))
		}
	}
}
