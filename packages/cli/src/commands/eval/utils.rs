use std::io::{self, prelude::Write};

// pub const EOF: usize = 0;

pub fn prompt(txt: &str) -> io::Result<()> {
	print!("{}", txt);
	io::stdout().flush()
}
