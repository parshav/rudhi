extern crate chrono;
extern crate dirs;

mod routine;
mod story;

use routine::Routine;
use story::Story;
use std::fs::File;
use std::fs;
use chrono::prelude::*;

fn main() {

	// load existing -> all in home directory
	// states -> empty, some

	let mut story = get_story().expect("Error in story");

	let test_routine = Routine {
		id: 2,
		name: String::from("Cooking"),
		last_done: Local::now()
	};
	//last_done: Local.ymd(2020, 1, 1).and_hms(0, 0, 0)
	//Local::now()
	story.add_routine(test_routine);

	println!("Story after add : {}", story);
	
	match story.routines.len() {
		0 => {
			println!("It was empty");
		}
		_ => {
			println!("Not empty");
		}
	}
}

fn print_routines(routines: &Vec<Routine>) {
	routines.iter().for_each(|routine| {
		println!("{}", routine);	
	});
}

fn get_story() -> Option<Story> {
	let mut home_story = dirs::home_dir().expect("Error in getting Home Dir");
		home_story.push(".rudhi");
		home_story.push("story");
		let exists = home_story.exists();
		if exists {
			let content = fs::read_to_string(home_story).expect("Error reading file");
			let story: Story = serde_json::from_str(&content).expect("Error in deserializing");
			return Some(story);
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
