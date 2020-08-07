extern crate chrono;

use chrono::prelude::*;
use std::fmt;

use serde_derive::{Serialize, Deserialize};

fn main() {

	// load existing -> all in home directory
	// states -> empty, some

	
	let routines = dummy_routines_data();
	print_routines(&routines);

}

fn print_routines(routines: &[Routine]) {
	routines.iter().for_each(|routine| {
		println!("{}", routine);	
	});
}

fn dummy_routines_data() -> [Routine; 3] {

	return [
		Routine {
			name: String::from("Running"),
			last_done: Local.ymd(2020, 7, 30)
		},
		Routine {
			name: String::from("Reading"),
			last_done: Local.ymd(2020, 7, 31)
		},
		Routine {
			name: String::from("Coding"),
			last_done: Local.ymd(2020, 8, 3)
		}
	]
}

#[derive(Serialize, Deserialize)]
struct Routine {
	name: String,
	last_done: Date<Local>
	// maybe an array of last done or notes for each time done
}

impl Routine {

	fn store_config(&self) {
		let json = serde_json::to_string(&self).expect("Could not convert to json");
		println!("Stored json : {}", json);
	}
}

impl fmt::Display for Routine {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  		write!(f, "{} last done on {}", self.name, self.last_done)
    }
}

/* Will be done later
struct Todo {
	name: String,
	added: Date<Local> // should be date
}

struct Thought {
	name: String,
	description: String,
	added: Date<Local>
}
*/
