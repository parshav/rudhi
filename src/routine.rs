use chrono::prelude::*;
use std::fmt;

pub struct Routine {
	name: String,
	last_done: Date<Local>
	// maybe an array of last done or notes for each time done
}

impl fmt::Display for Routine {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  		write!(f, "{} last done on {}", self.name, self.last_done)
    }
}

// mock
pub fn dummy_routines_data() -> [Routine; 3] {

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
