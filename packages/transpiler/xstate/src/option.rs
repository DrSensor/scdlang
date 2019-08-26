use strum_macros::*;

#[derive(AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Key {
	Output,
	ExportName,
}

#[derive(AsRefStr, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Output {
	JSON,
	TypeScript,
	JavaScript,
}
