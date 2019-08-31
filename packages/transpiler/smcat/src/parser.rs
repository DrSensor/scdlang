use super::{schema::*, utils::*, *};
use scdlang::{
	prelude::*,
	semantics::{Found, Kind},
	Scdlang,
};
use std::{fmt, mem::ManuallyDrop};

impl fmt::Display for Machine<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let json = serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?;
		write!(f, "{}", json.trim())
	}
}

impl<'a> Parser<'a> for Machine<'a> {
	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		use StateType::*;
		let mut schema = Coordinate::default();
		let get = |key| builder.get(key);

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					let (color, note) = match expr.semantic_check()? {
						Found::Error(ref message) if !builder.semantic_error => (
							Some("red".to_string()),
							Some(message.split('\n').map(|s| s.trim_start_matches(' ').to_string()).collect()),
						),
						_ => (None, None),
					};
					let (event, cond, action) = (
						expr.event().map(|e| e.into()),
						expr.guard().map(|e| e.into()),
						expr.action().map(|e| e.into()),
					);

					schema.states.merge(&{
						let (mut current, mut next) = (
							expr.current_state().into_type(Regular),
							expr.next_state().map(|s| s.into_type(Regular)),
						);
						if let/* mark error */Some(color) = &color {
							current.with_color(color);
							next = next.map(|mut s| s.with_color(color).clone())
						}
						use option::Mode;
						match next {
							Some(next) => vec![current, next], // external transition
							None if get(&Config::Mode) == Some(Mode::BlackboxState.as_ref()) => vec![/*WARNING:wasted*/], // ignore anything inside state
							None => {
								if let (Some(event), Some(action)) = (event.as_ref(), action.as_ref()) {
									current.actions = Some(vec![ActionType {
										r#type: ActionTypeType::Activity,
										body: match cond.as_ref() {
											None => format!("{} / {}", event, action),
											Some(cond) => format!("{} [{}] / {}", event, cond, action),
										},
									}]);
								}
								vec![current] // internal transition
							}
						}
					});

					if let/* external transition */Some(next_state) = expr.next_state() {
						#[rustfmt::skip]
						let transition = Transition {
							from: expr.current_state().into(),
							to: next_state.into(),
							label: if event.is_some() || cond.is_some() || action.is_some() {
								let action_cond = action.is_some() || cond.is_some();
								let (event, cond, action) = (event.clone(), cond.clone(), action.clone());
								Some(format!( // add spacing on each token
									"{on}{is}{run}",
									on = event.map(|event| format!("{}{spc}", event, spc = if action_cond { " " } else { "" },))
										.unwrap_or_default(),
									is = cond.map(|guard| format!("[{}]{spc}", guard, spc = if action.is_some() { " " } else { "" },))
										.unwrap_or_default(),
									run = action.map(|act| format!("/ {}", act)).unwrap_or_default()
								))
							} else { None }, event, cond, action, color, note
						};
						match &mut schema.transitions {
							Some(transitions) => transitions.push(transition),
							None => schema.transitions = Some(vec![transition]),
						};
					}
				}
				_ => unimplemented!("TODO: implement the rest on the next update"),
			}
		}

		Ok(Machine { schema, builder })
	}

	fn configure(&mut self) -> &mut dyn Builder<'a> {
		&mut self.builder
	}

	fn parse(&mut self, source: &str) -> Result<(), DynError> {
		self.clean_cache()?;
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		Ok(self.schema = ast.schema.to_owned()) // FIXME: expensive clone
	}

	fn insert_parse(&mut self, source: &str) -> Result<(), DynError> {
		let mut ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		self.schema.states.merge(&ast.schema.states);
		match (&mut self.schema.transitions, &mut ast.schema.transitions) {
			(Some(origin), Some(parsed)) => origin.extend_from_slice(parsed),
			(None, _) => self.schema.transitions = ast.schema.transitions.to_owned(),
			_ => {}
		}
		Ok(())
	}
}
