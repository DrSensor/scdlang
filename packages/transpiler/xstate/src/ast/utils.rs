pub mod span {
	use crate::ast::Transition;
	use from_pest::FromPest;
	use pest::{Parser, Span};
	use scdlang_core::{Rule, Scdlang};
	use serde_json::Value;
	use std::collections::HashMap;

	pub fn into_string(span: Span) -> String {
		span.as_str().to_string()
	}

	pub fn into_pair(span: Span) -> HashMap<String, Transition> {
		let mut json = HashMap::new();
		if let Ok(expressions) = Scdlang::parse(Rule::expression, span.as_str()) {
			for expr in expressions {
				let mut inner = expr.into_inner();
				|| -> Option<_> {
					let state_name = inner.next()?.as_str().to_string();
					let transition = Transition::from_pest(&mut inner).expect("infallible");
					json.insert(state_name, transition)
				}();
			}
		}
		json
	}

	pub fn into_json(span: Span) -> HashMap<String, Value> {
		let mut json = HashMap::new();
		let event_name = "".to_string();
		let state_name = Value::String(span.as_str().to_string());
		json.insert(event_name, state_name);
		json
	}
}

#[cfg(test)]
mod test {
	// WARNING: only create the test case after pest-ast is stable❗
}
