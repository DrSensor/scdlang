#![allow(deprecated)]
use super::helper::{get, prelude::*};
use crate::semantics::{Action, StateType, Transition, TransitionType};

impl<'t> From<TokenPair<'t>> for Transition<'t> {
	fn from(pair: TokenPair<'t>) -> Self {
		let (mut lhs, mut rhs) = ("", "");
		let mut ops = Rule::EOI;
		let mut event = None;
		let mut action = None;

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
				Rule::internal_transition => {
					let (state, ev, act) = get::arrowless_transition(span);
					lhs = state;
					rhs = lhs;
					event = Some(ev);
					action = Some(act);
				}
				Rule::trigger => event = Some(get::trigger(span)),
				Rule::action => action = Some(Action { name: get::action(span) }),
				_ => unreachable!(
					"Rule::{:?} not found when determine the lhs, rhs, and operators",
					span.as_rule()
				),
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
			_ => unreachable!(
				"Rule::{:?} not found when determine the current, next, and type of the State",
				&ops
			),
		};

		// register into Transition graph
		Transition {
			from: current_state,
			to: next_state,
			at: event,
			run: action,
			kind: transition_type,
		}
	}
}
