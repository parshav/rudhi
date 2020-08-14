use crate::routine::Routine;
use std::fmt;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Story {
	pub routines: Vec<Routine>
}

impl Story {

	pub fn add_routine(&mut self, routine: Routine) {
		self.routines.push(routine);
	}
}

impl fmt::Display for Story {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		print!("\nRoutinely stuff : ");
		self.routines.iter().for_each(|r| {
			write!(f, "\n{}", r);	
		});
		Ok(())
    }
}
