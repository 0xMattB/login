use crate::modules::file;
use crate::modules::console;

pub fn load_data(filename: &str) -> Option<String> {
	if let Some(data) = file::read(filename) {
		Some(String::from(data))
	} else {
		None
	}
}

pub fn save_data(filename: &str, data: &str) -> bool {
	if let Ok(()) = file::write(filename, data) {
		true
	} else {
		false
	}
}

pub fn receive_email(prompt: &str) -> Option<String> {
	loop {
		console::output(prompt, false);
		
		if let Ok(s) = console::input() {
			let s = s.trim();
			let s = s.to_lowercase().to_string();

			if validate_email(&s) {
				return Some(s);
			}
		} else {
			return None;
		}
		
		console::output("Invalid input\n", true);
	}
}

pub fn receive_password(prompt: &str) -> Option<String> {
	console::output(prompt, false);
	
	if let Ok(s) = console::input() {
		let s = s.trim();
		Some(String::from(s))
	} else {
		return None
	}
}

pub fn receive_accept(prompt: &str) -> Option<String> {
	console::output(prompt, false);
	
	if let Ok(s) = console::input() {
		let s = s.trim();
		let s = s.to_lowercase();
		Some(String::from(s))
	} else {
		return None
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