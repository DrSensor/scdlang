use crate::{print::*, prompt};
use colored::*;
use std::*;

pub enum Error {
	IO(io::Error),
	Parse(String),
	Whatever(Box<dyn error::Error>),
}

pub fn global_reporting(err: Error) {
	let print = PRINTER("haskell", Mode::Error);
	let error = |message: &str| format!("{} {}", prompt::ERROR.red().bold(), message.red());
	let prompting = |message: &str| eprintln!("{}", error(message));
	let pprompting = |message: &str, header: &str| print.string_with_header(message, &error(header)).unwrap();
	// TODO: ☝️ should output to stderr instead of stdout

	match err {
		Error::Whatever(msg) => prompting(&msg.to_string()),
		Error::Parse(msg) => pprompting(&msg, "can't parse"),
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
