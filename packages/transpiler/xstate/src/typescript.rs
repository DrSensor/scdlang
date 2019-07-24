use crate::{option, Builder, DynError, Machine};
use serde_json;

pub const HELPER: &str = "\n
type EventInState<Machine extends any> = {
    readonly [State in keyof Machine[\"states\"]]: keyof Machine[\"states\"][State][\"on\"];
}
";

impl Machine<'_> {
	pub(super) fn to_typescript(&self) -> Result<String, DynError> {
		if let Some(export_name) = self.builder.get(option::EXPORT) {
			let json = serde_json::to_string_pretty(&self.schema)?;
			let mut fsm_interface = format!("r#type {}Schema = {}", export_name, json);

			fsm_interface.push_str(&format!(
				"\n\nr#type {}Event = {}",
				export_name,
				self.schema
					.states
					.keys()
					.map(|key| format!("{{ type: EventIn[\"{}\"] }}", key))
					.collect::<Vec<String>>()
					.join(" | ")
			));
			// TODO: include nested states
			fsm_interface.push_str(&format!(
				"\n\nr#type {name}State = keyof {name}[\"states\"]",
				name = export_name,
			));
			fsm_interface.push_str(&format!("\n\nr#type EventIn = EventInState<{}>", export_name));

			fsm_interface.push_str(HELPER);
			Ok(fsm_interface)
		} else {
			panic!("\"{}\" must be defined", option::EXPORT)
		}
	}
}
