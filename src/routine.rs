use serde_derive::{Serialize, Deserialize};
use chrono::prelude::*;
use std::fmt;
use chrono::{DateTime, TimeZone};


#[derive(Serialize, Deserialize)]
pub struct Routine {
	name: String,
	last_done: DateTime<Local>
	// maybe an array of last done or notes for each time done
}

impl fmt::Display for Routine {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  		write!(f, "{} last done on {}", self.name, self.last_done.format("%a %b%e, %Y"))
    }
}

// mock
pub fn dummy_routines_data() -> [Routine; 3] {

	return [
		Routine {
			name: String::from("Running"),
			last_done: Local.ymd(2020, 8, 9).and_hms(14, 0, 0)
		},
		Routine {
			name: String::from("Reading"),
			last_done: Local.ymd(2020, 8, 10).and_hms(0, 0, 0)
		},
		Routine {
			name: String::from("Coding"),
			last_done: Local::now()
		}
	]
}

impl Routine {

	pub fn store_config(&self) {
		let json = serde_json::to_string(&self).expect("Could not convert to json");
		println!("Stored json : {}", json);
	}
}

