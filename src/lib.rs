pub mod modules;

use crate::modules::file;
use crate::modules::console;
use crate::modules::encrypt;
use crate::modules::accounts::Accounts;

use std::process;

pub fn run(filename: &str) {
	let data = attempt_to_load_data(filename).unwrap_or_else(|| process::exit(0));
	let mut accounts = attempt_to_create_accounts(&data).unwrap_or_else(|| process::exit(0));
	
	let email = get_email(true).unwrap_or_else(|| process::exit(0));
	
	if email == "new" {
		let mut email: String;
		let password: String;
		
		loop {
			email = get_email(true).unwrap_or_else(|| process::exit(0));
			
			if accounts.check_email(&email) {
				console::output("Email already in use\n", true);
			} else {
				break;
			}
		}
		
		loop {
			let pw1 = get_password().unwrap_or_else(|| process::exit(0));
			let pw2 = get_password().unwrap_or_else(|| process::exit(0));
			
			if pw1 != pw2 {
				console::output("Passwords don't match!", true);
			} else {
				password = pw1;
				break;
			}
		}
		
		let password = encrypt::encrypt(&password);
		accounts.add(&email, &password);
		
		if let Some(new_data) = accounts.as_string() {
			if let Ok(()) = file::write(filename, &new_data) {
				console::output("Data updated", true);
			} else {
				console::output("Error writing data to file", true);
			}
		} else {
			console::output("Error converting data to string", true);
		}
	} else {
		let password = get_password().unwrap_or_else(|| process::exit(0));
		let password = encrypt::encrypt(&password);
		
		if accounts.check(&email, &password) {
			console::output("\nSign-in Successful!", true);
		} else {
			console::output("\nInvalid email or password", true);
		}
	}
}

fn attempt_to_load_data(filename: &str) -> Option<String> {
	if let Some(data) = file::read(filename) {
		return Some(String::from(data));
	} else {
		console::output(&format!["{filename} doesn't exist - create? (y/n): "], false);
		
		match console::input() {
			Ok(s) => {
				let s = s.trim();
				
				if s == "Y" || s == "y" {
					return Some(String::from(""));
				} else {
					return None;
				}
			},
			Err(e) => {
				console::output("", true);
				console::output(e, true);
				return None;
			},
		}
	}
}

fn attempt_to_create_accounts(data: &str) -> Option<Accounts> {
	match Accounts::new(data) {
		Ok(a) => {
			Some(a)
		},
		Err(e) => {
			console::output(e, true);
			None
		},
	}
}

fn get_email(allow_new: bool) -> Option<String> {
	loop {
		let read: String;
		
		if allow_new {
			console::output("Email (or 'new'): ", false);
		} else {
			console::output("Email: ", false);
		}
		
		if let Ok(s) = console::input() {
			let s = s.trim();
			read = s.to_lowercase().clone();
		} else {
			return None;
		}
		
		match read.as_str() {
			"new" => {
				if allow_new {
					return Some(read);
				} else {
					console::output("Invalid input\n", true);
				}
			},
			_ => {
				if validate_email(&read) {
					return Some(read);
				} else {
					console::output("Invalid input\n", true);
				}
			},
		}
	}
}

fn validate_email(email: &str) -> bool {
	if let Some(c_at) = email.find('@') {
		if let Some(c_dot) = email.find('.') {
			if c_at < c_dot {
				return true;
			}
		}
	}
	
	false
}

fn get_password() -> Option<String> {
	console::output("Password: ", false);
	
	if let Ok(s) = console::input() {
		let s = s.trim();
		Some(String::from(s))
	} else {
		return None
	}
}