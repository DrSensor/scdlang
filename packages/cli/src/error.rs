use crate::prompt;
use std::*;

pub enum Error {
	IO(io::Error),
	Parse(String),
}

pub fn global_reporting(err: Error) {
	let prompt = prompt::ERROR;
	let prompting = |message: &str| eprintln!("{} {}", prompt, message);

	match err {
		Error::Parse(msg) => prompting(&msg),
		Error::IO(msg) => {
			let no_oserr = msg.to_string().replace("os error ", "");
			let arr_msg: Vec<&str> = no_oserr.split(' ').collect();
			prompting(&arr_msg[..arr_msg.len() - 1].join(" "));
			process::exit(msg.raw_os_error().unwrap())
		}
	}

	process::exit(-1)
}
