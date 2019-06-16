use crate::{print::*, prompt};
use atty::Stream;
use colored::*;
use std::*;

pub enum Error {
	IO(io::Error),
	Parse(String),
	Whatever(Box<dyn error::Error>),
}

// TODO: impl From<std::boxed::Box<dyn std::error::Error>> for Error

impl Error {
	pub fn report(err: Error, default_exit_code: Option<i32>) {
		let print = PRINTER("haskell").change(Mode::Error);
		let error = |message: &str| {
			if atty::is(Stream::Stderr) {
				format!("{} {}", prompt::ERROR.red().bold(), message.red())
			} else {
				format!("{} {}", prompt::ERROR, message)
			}
		};
		let prompting = |message: &str| eprintln!("{}", error(message));
		let pprompting = |message: &str, header: &str| {
			if atty::is(Stream::Stderr) {
				print.string_with_header(message, &error(header)).unwrap()
			} else {
				eprintln!("{}\n---\n{}\n---", error(header), message)
			}
		};

		match err {
			Error::Whatever(msg) => prompting(&msg.to_string()),
			Error::Parse(msg) => pprompting(&msg, "can't parse"),
			Error::IO(msg) => {
				let sanitize_msg = remove_os_error(msg.to_string());
				prompting(&sanitize_msg);
				if default_exit_code.is_some() {
					process::exit(msg.raw_os_error().unwrap())
				}
			}
		}

		if let Some(exit_code) = default_exit_code {
			process::exit(exit_code)
		}
	}
}

fn remove_os_error(message: String) -> String {
	let no_oserr = message.replace("os error ", "");
	let arr_msg: Vec<&str> = no_oserr.split(' ').collect();
	arr_msg[..arr_msg.len() - 1].join(" ")
}
