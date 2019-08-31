use super::{schema::*, *};
use scdlang::{prelude::*, semantics::Kind, Scdlang};
use std::{fmt, mem::ManuallyDrop};
use voca_rs::case::{camel_case, shouty_snake_case};

impl fmt::Display for Machine<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let get = |key| self.builder.get(key).ok_or(fmt::Error);
		let (output, export_name) = (get(&Config::Output)?, get(&Config::ExportName));
		match output {
			"json" | "javascript" | "js" => {
				let json = serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?;
				if let "javascript" | "js" = output {
					return write!(f, "export const {} = {}", export_name?, json.trim());
				}
				write!(f, "{}", json.trim())
			}
			"dts" | "typescript" | "ts" => {
				let mut dts = self.to_typescript().map_err(|_| fmt::Error)?;
				if output == "dts" {
					dts = dts.replace("export type ", "type ");
				}
				write!(f, "{}", dts.trim())
			}
			_ => Ok(()),
		}
	}
}

impl<'a> Parser<'a> for Machine<'a> {
	fn try_parse(source: &str, builder: Scdlang<'a>) -> Result<Self, DynError> {
		let mut schema = StateChart::default();

		for kind in builder.iter_from(source)? {
			match kind {
				Kind::Expression(expr) => {
					let (current_state, next_state) = (
						expr.current_state().map(camel_case),
						expr.next_state().map(|e| e.map(camel_case)),
					);
					let (event_name, guard) = (
						expr.event().map(|e| e.map(shouty_snake_case)).unwrap_or_default(),
						expr.guard().map(|e| e.map(camel_case)),
					);
					let action = expr.action().map(|e| e.map(camel_case));

					let (not_obj, t_obj) = (
						guard.is_none() && action.is_none(),
						TransitionObject {
							target: next_state,
							actions: action,
							cond: guard,
						},
					);

					let transition = if not_obj {
						Transition::Target(t_obj.target.clone())
					} else {
						Transition::Object(t_obj.clone())
					};

					let t = schema.states.entry(current_state).or_insert(State {
						// TODO: waiting for map macros https://github.com/rust-lang/rfcs/issues/542
						on: [(event_name.to_string(), transition.clone())].iter().cloned().collect(),
					});
					t.on.entry(event_name.to_string())
						.and_modify(|target| {
							match target {
								Transition::ListObject(objects) => objects.push(t_obj),
								_ if target != &transition => {
									*target = Transition::ListObject(vec![
										(if let Transition::Object(obj) = target { obj } else { &t_obj }).clone(),
										t_obj,
									])
								}
								_ => {}
							};
						})
						.or_insert(transition);
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
		let ast = ManuallyDrop::new(Self::try_parse(source, self.builder.to_owned())?);
		for (current_state, transition) in ast.schema.states.to_owned(/*FIXME: expensive clone*/) {
			self.schema
				.states
				.entry(current_state)
				.and_modify(|t| t.on.extend(transition.on.clone()))
				.or_insert(transition);
		}
		Ok(())
	}
}
