use crate::routine::Routine;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Story {
	pub routines: Vec<Routine>
}
