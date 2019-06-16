use crate::semantics;
use semantics::{Transition, TransitionType};

impl<'i> IntoIterator for Transition<'i> {
	type Item = Transition<'i>;
	type IntoIter = TransitionIterator<'i>;

	fn into_iter(mut self) -> Self::IntoIter {
		TransitionIterator(match self.kind {
			TransitionType::Normal => [self].to_vec(),
			TransitionType::Toggle => {
				self.kind = TransitionType::Normal;
				let (mut left, right) = (self.clone(), self);
				left.from = right.to.clone();
				left.to = right.from.clone();
				[left, right].to_vec()
			}
			TransitionType::Loop { transient } => {
				/* A ->> B @ C */
				if self.from.name != self.to.name {
					let (mut self_loop, mut normal) = (self.clone(), self);
					self_loop.from = self_loop.to.clone();
					normal.kind = TransitionType::Normal;
					normal.at = if transient { None } else { normal.at };
					[self_loop, normal].to_vec()
				}
				/* ->> B @ C */
				else {
					[self].to_vec() // reason: see Symbol::double_arrow::right => (..) in convert.rs
				}
			}
			TransitionType::Inside { .. } => unreachable!("TODO: when support StateType::Compound"),
		})
	}
}

pub struct TransitionIterator<'i>(Vec<Transition<'i>>);

impl<'i> Iterator for TransitionIterator<'i> {
	type Item = Transition<'i>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}
