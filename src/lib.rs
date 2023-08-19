pub mod modules;

use crate::modules::io;
use crate::modules::console;
use crate::modules::encrypt;
use crate::modules::accounts::Accounts;

use std::process;

pub fn run(filename: &str) {
	let mut data = String::from("");

	if let Some(read_data) = io::load_data(filename) {
		data = read_data;
	} else {
		let response = io::receive_accept("File not found - create new file? (y/n): ").unwrap_or_else(|| process::exit(0));
		if response != "y" {
			process::exit(0);
		}
	}
	
	let mut accounts = create_accounts(&data).unwrap_or_else(|| process::exit(0));
	let email = io::receive_email("Email: ").unwrap_or_else(|| process::exit(0));
	
	if accounts.check_email(&email) {
		let password = io::receive_password("Password: ").unwrap_or_else(|| process::exit(0));
		let password = encrypt::encrypt(&password);
		
		if accounts.check(&email, &password) {
			console::output("\nData Found", true);
		} else {
			console::output("\nData Not Found", true);
		}
	} else {
		let response = io::receive_accept("Email not found - create new account? (y/n): ").unwrap_or_else(|| process::exit(0));
		if response == "y" {
			let mut pw1: String;
			let mut pw2: String;
			
			loop {
				pw1 = io::receive_password("Password: ").unwrap_or_else(|| process::exit(0));
				pw2 = io::receive_password("Password: ").unwrap_or_else(|| process::exit(0));
				
				if pw1 == pw2 {
					break;
				} else {
					console::output("Passwords don't match\n", true);
				}
			}
			
			let password = encrypt::encrypt(&pw1);
			
			if !accounts.add(&email, &password) {
				console::output("Email already in use\n", true);
			} else {
				let new_data = accounts.as_string().unwrap_or_else(|| process::exit(0));
				
				if io::save_data(filename, &new_data) {
					console::output("\nData updated", true);
				} else {
					console::output("\nError saving data", true);
				}
			}
		}
	}
}

fn create_accounts(data: &str) -> Option<Accounts> {
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
