#![macro_use]

macro_rules! filtrate {
	($condition:expr) => {
		|pair| match Transition::try_from(pair.to_owned()) {
			Ok(transition) => $condition(transition),
			Err(_err) => false,
		}
	};
}
