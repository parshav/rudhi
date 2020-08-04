extern crate chrono;

fn main() {

	// load existing -> all in home directory
	// states -> empty, some
	// dummy data  for now

}

fn dummy_data() -> [Routine; 1] {

	return [
		Routine {
			name: String::from("Running"),
			last_done: String::from("30/07/2020")
		}
	]
}

struct Routine {
	name: String,
	last_done: String // should be date
	// maybe an array of last done or notes for each time done
}

struct Todo {
	name: String,
	added: String // should be date
}

struct Thought {
	name: String,
	description: String
}
