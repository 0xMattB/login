use std::env;

use login::modules::console;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		login::run(&args[1]);
	} else {
		console::output("invalid number of arguments; usage:\nlogin.exe (filename.ext)", true);
	}
}
