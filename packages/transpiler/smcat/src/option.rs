use strum_macros::*;

#[derive(AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Key {
	Mode,
}

#[derive(AsRefStr, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Mode {
	BlackboxState,
}
