use crate::prompt;
use std::*;

pub enum Error {
	IO(io::Error),
	Parse(String),
	Whatever(Box<dyn error::Error>),
}

pub fn global_reporting(err: Error) {
	let prompting = |message: &str| eprintln!("{} {}", prompt::ERROR, message);

	match err {
		Error::Whatever(msg) => prompting(&msg.to_string()),
		Error::Parse(msg) => prompting(&msg),
		Error::IO(msg) => {
			let sanitize_msg = remove_os_error(msg.to_string());
			prompting(&sanitize_msg);
			process::exit(msg.raw_os_error().unwrap())
		}
	}

	process::exit(-1)
}

fn remove_os_error(message: String) -> String {
	let no_oserr = message.replace("os error ", "");
	let arr_msg: Vec<&str> = no_oserr.split(' ').collect();
	arr_msg[..arr_msg.len() - 1].join(" ")
}
