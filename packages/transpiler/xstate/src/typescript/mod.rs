use super::*;
use serde_json;

const TEMPLATE: &str = include_str!("schema.ts");

impl Machine<'_> {
	pub(super) fn to_typescript(&self) -> Result<String, DynError> {
		if let Some(export_name) = self.builder.get(&Config::ExportName) {
			Ok(TEMPLATE
				.replace("$name", export_name)
				.replace("$schema", &serde_json::to_string_pretty(&self.schema)?)
				.replace(
					"/*each*/EventIn",
					&self
						.schema
						.states
						.keys()
						.map(|key| format!(r#"EventIn["{state}"]"#, state = key))
						.collect::<Vec<_>>()
						.join(" | "),
				)
				.replace("//@ts-ignore", ""))
		} else {
			Err(format!("\"{config}\" must be defined", config = Config::ExportName.as_ref()).into())
		}
	}
}
