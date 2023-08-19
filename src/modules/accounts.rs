use std::collections::HashMap;

pub struct Accounts {
	list: HashMap<String, String>,
}

impl Accounts {
	pub fn new(data: &str) -> Result<Accounts, &'static str> {
		if let Some(list) = Self::parse(data) {
			Ok(
				Accounts {
					list,
				}
			)
		} else {
			Err("accounts::new(): invalid file data")
		}
	}
	
	pub fn add(&mut self, email: &str, password: &str) -> bool {
		if Self::check_email(self, email) {
			false
		} else {
			self.list.insert(String::from(email), String::from(password));
			true
		}
	}
	
	pub fn remove(&mut self, email: &str, _password: &str) -> bool {
		if let Some(_) = self.list.remove(email) {
			true
		} else {
			false
		}
	}
	
	pub fn check(&self, email: &str, password: &str) -> bool {
		if let Some(pw) = Self::get_password(self, email) {
			if pw == password {
				return true;
			}
		}
		
		false
	}
	
	pub fn check_email(&self, email: &str) -> bool {
		self.list.contains_key(email)
	}
	
	pub fn get_password(&self, email: &str) -> Option<String> {
		if let Some((_, value)) = self.list.get_key_value(email) {
			return Some(String::from(value));
		}
		
		None
	}
	
	pub fn update(&mut self, email: &str, pw_old: &str, pw_new: &str) -> bool {
		if let Some((_, value)) = self.list.get_key_value(email) {
			if value == pw_old {
				self.list.remove(email);
				self.list.insert(String::from(email), String::from(pw_new));
				return true;
			}
		}
		
		false
	}
	
	pub fn as_string(&self) -> Option<String> {
		let mut line = String::new();
		
		if !self.list.is_empty() {
			for (key, value) in &self.list {
				line.push_str(&format!["{}\t{}\n", key, value]);
			}
			
			Some(line)
		} else {
			None
		}
	}
	
	fn parse(data: &str) -> Option<HashMap<String, String>> {
		let mut list = HashMap::new();
		
		for line in data.lines() {
			let entries: Vec<_> = line.split_whitespace().collect();
			
			if entries.len() == 2 {
				list.insert(String::from(entries[0]), String::from(entries[1]));
			} else {
				list.clear();
				return None;
			}
		}
		
		Some(list)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn accounts_new_test() {
		if let Ok(_) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(true);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_add_success_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(accounts.add("xyz@domain.com", "pAsSwOrD"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_add_error_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(!accounts.add("abc@domain.com", "pAsSwOrD"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_remove_success_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(accounts.remove("abc@domain.com", "pAsSwOrD"));
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn accounts_remove_error_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(!accounts.remove("xyz@domain.com", "pAsSwOrD"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_check_success_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(accounts.check("abc@domain.com", "abc123"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_check_error_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(!accounts.check("abc@domain.com", "abc12"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_check_email_success_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(accounts.check_email("abc@domain.com"));
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn accounts_check_email_error_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			assert!(!accounts.check_email("xyz@domain.com"));
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_get_password_success_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			if let Some(pw) = accounts.get_password("def@domain.com") {
				assert_eq!(pw, String::from("def456"));
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_get_password_error_test() {
		if let Ok(accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			if let Some(pw) = accounts.get_password("def@domain.com") {
				assert_ne!(pw, String::from("def45"));
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_update_success_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			if accounts.update("ghi@domain.com", "ghi789", "tuv987") {
				if let Some(s) = accounts.as_string() {
					let mut v: Vec<&str> = s.lines().collect();
					v.sort();
					assert_eq!(v, vec!["abc@domain.com\tabc123", "def@domain.com\tdef456", "ghi@domain.com\ttuv987"]);
				} else {
					assert!(false);
				}
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}
	
	#[test]
	fn accounts_update_error_test() {
		if let Ok(mut accounts) = Accounts::new("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			if !accounts.update("ghi@domain.com", "ghi78", "tuv987") {
				assert!(true);
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_as_string_test() {
		let test_data = "abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n";
		
		if let Ok(accounts) = Accounts::new(test_data) {
			if let Some(s) = accounts.as_string() {
				let mut v: Vec<&str> = s.lines().collect();
				v.sort();
				assert_eq!(v, vec!["abc@domain.com\tabc123", "def@domain.com\tdef456", "ghi@domain.com\tghi789"]);
			} else {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}

	#[test]
	fn accounts_parse_test() {
		if let Some(list) = Accounts::parse("abc@domain.com\tabc123\ndef@domain.com\tdef456\nghi@domain.com\tghi789\n") {
			let mut v: Vec<(&str, &str)> = Vec::new();
			
			for (key, value) in &list {
				v.push((key, value));
			}
			
			v.sort();
			assert_eq!(v, vec![("abc@domain.com", "abc123"), ("def@domain.com", "def456"), ("ghi@domain.com", "ghi789")]);
		} else {
			assert!(false);
		}	
	}
}