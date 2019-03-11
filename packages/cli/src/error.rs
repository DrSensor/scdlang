use std::*;

pub enum Error {
	IO(io::Error),
	Parse(String),
}

pub fn global_reporting(err: Error) {
	let prompt = "ERROR:";
	let prompting = |message: &str| println!("{} {}", prompt, message);

	match err {
		Error::Parse(msg) => prompting(&msg),
		Error::IO(msg) => prompting(&msg.to_string()),
	}
}
