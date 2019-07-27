use crate::{option, Builder, DynError, Machine};
use serde_json;
use std::fmt::{self, Write};

impl Machine<'_> {
	pub(super) fn to_typescript(&self) -> Result<String, DynError> {
		if let Some(export_name) = self.builder.get(option::EXPORT) {
			let json = serde_json::to_string_pretty(&self.schema)?;
			let mut fsm_interface = format!("type {name} = {expr}", name = export_name, expr = json);

			fsm_interface.set_root_state(export_name)?;
			fsm_interface.set_schema(export_name)?;
			fsm_interface.set_event_selector(export_name)?;
			fsm_interface.set_event(
				export_name,
				&self
					.schema
					.states
					.keys()
					.map(|key| format!("EventIn[\"{}\"]", key))
					.collect::<Vec<String>>()
					.join(" | "),
			)?;

			fsm_interface.push_str(EVENT_HELPER);
			Ok(fsm_interface)
		} else {
			panic!("\"{}\" must be defined", option::EXPORT)
		}
	}
}

impl DeclarationHelper for String {}
trait DeclarationHelper: Write {
	fn set_root_state(&mut self, name: &str) -> fmt::Result {
		write!(self, "\n\nr#type {name}State = keyof {name}[\"states\"]", name = name)
	}
	fn set_event_selector(&mut self, name: &str) -> fmt::Result {
		write!(self, "\n\nr#type EventIn = EventInState<{name}>", name = name)
	}
	#[rustfmt::skip]
	#[allow(dead_code)]
	fn set_state_selector(&mut self, name: &str, interface: &str) -> fmt::Result {
		write!(self, "\n\nr#type {name}StateIn = StateInState<{state}>", name = name, state = interface)
	}
}

impl Declaration for String {}
trait Declaration: Write {
	#[rustfmt::skip]
	fn set_schema(&mut self, name: &str) -> fmt::Result {
		write!(self, "\n
r#type {name}Schema = {{
  \"states\": {{ [source in {name}State]: {{}} }}
}}", 			name = name
		)
	}
	fn set_event(&mut self, name: &str, expr: &str) -> fmt::Result {
		write!(self, "\n\nr#type {name}Event = {{ type: {expr} }}", name = name, expr = expr)
	}
}

pub const EVENT_HELPER: &str = "\n
type EventInState<Machine extends any> = {
  readonly [source in keyof Machine[\"states\"]]: keyof Machine[\"states\"][source][\"on\"];
}
";

#[allow(dead_code)]
pub const STATE_HELPER: &str = "\n
type StateInState<Machine extends any> = {
  readonly [source in keyof Machine[\"states\"]]: keyof Machine[\"states\"][source][\"states\"];
}
";

// TODO: consider using `include_str!` to separate each type in template file
// It will give you syntax highlighting and auto-completion
