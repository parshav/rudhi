extern crate chrono;

mod routine;

use routine::Routine;

fn main() {

	// load existing -> all in home directory
	// states -> empty, some

	
	let routines = routine::dummy_routines_data();
	print_routines(&routines);
}

fn print_routines(routines: &[Routine]) {
	routines.iter().for_each(|routine| {
		println!("{}", routine);	
		routine.store_config();
	});
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
