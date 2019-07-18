use crate::semantics;
use semantics::{Transition, TransitionType};
use std::iter::FromIterator;

impl<'i> IntoIterator for Transition<'i> {
	type Item = Transition<'i>;
	type IntoIter = TransitionIterator<'i>;

	fn into_iter(mut self) -> Self::IntoIter {
		TransitionIterator(match self.kind {
			/*FIXME: iterator for internal transition*/
			TransitionType::Normal | TransitionType::Internal => [self].to_vec(),
			TransitionType::Toggle => {
				self.kind = TransitionType::Normal;
				let (mut left, right) = (self.clone(), self);
				left.from = right.to.clone().expect("not Internal");
				left.to = Some(right.from.clone());
				[left, right].to_vec()
			}
			TransitionType::Loop { transient } => {
				/* A ->> B @ C */
				if self.from.name != self.to.as_ref().expect("not Internal").name {
					let (mut self_loop, mut normal) = (self.clone(), self);
					self_loop.from = self_loop.to.as_ref().expect("not Internal").clone();
					normal.kind = TransitionType::Normal;
					normal.at = if transient { None } else { normal.at };
					[normal, self_loop].to_vec()
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

impl<'i> FromIterator<Transition<'i>> for [Transition<'i>; 2] {
	fn from_iter<T>(transition: T) -> Self
	where
		T: IntoIterator<Item = Transition<'i>>,
	{
		let mut iter = transition.into_iter();
		[iter.next().unwrap(), iter.next().unwrap()]
	}
}

impl<'i> FromIterator<Transition<'i>> for [Transition<'i>]
where
	Self: Sized,
{
	fn from_iter<T>(_transition: T) -> Self
	where
		T: IntoIterator<Item = Transition<'i>>,
	{
		unimplemented!("TODO: on the next update")
	}
}
