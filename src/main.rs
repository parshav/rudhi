extern crate chrono;
extern crate dirs;

mod routine;

use std::env;
use routine::Routine;
use std::fs::File;
use std::fs;

fn main() {

	// load existing -> all in home directory
	// states -> empty, some
	let routines = get_story();
	print_routines(&routines);

	match routines.len() {
		0 => {
			println!("It was empty");
		}
		_ => {
			println!("Not empty");
		}
	}

	let routine = read_files_to_routine().unwrap();
	println!("Stored routine : {}", routine);
}

fn print_routines(routines: &Vec<Routine>) {
	routines.iter().for_each(|routine| {
		println!("{}", routine);	
	});
}

fn get_story() -> Vec<Routine> {
	routine::dummy_routines_data().to_vec()
}

//should return Array of routine instead of just single
// also clean this up
fn read_files_to_routine() -> Option<Routine> {
	let mut home_story = dirs::home_dir().expect("Error in getting Home Dir");
	home_story.push(".rudhi");
	home_story.push("story");
	let exists = home_story.exists();
	if exists {
		let content = fs::read_to_string(home_story).expect("Error reading file");
		let routine: Routine = serde_json::from_str(&content).expect("Error in deserializing");
		return Some(routine);
	}
	println!("Home : {}", exists);
	return None;
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
