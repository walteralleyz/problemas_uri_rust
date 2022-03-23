use std::collections::hash_map::HashMap;
use std::env;

mod cut_the_sticks;
mod find_digits;
mod library_fine;
mod repeated_string;
mod sherlock_and_squares;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let challenges: HashMap<&str, fn()> = HashMap::from([
        ("find_digits", get_as_fn(find_digits::main)),
        ("sherlock_and_squares", get_as_fn(sherlock_and_squares::main)),
        ("library_fine", get_as_fn(library_fine::main)),
        ("cut_the_sticks", get_as_fn(cut_the_sticks::main)),
        ("repeated_string", get_as_fn(repeated_string::main)),
    ]);

    let option = args.get(2).expect("Challenge title expected!");

    challenges.get(&option.as_str()).unwrap()();
}

fn get_as_fn(f: fn() -> ()) -> fn() {
    f as fn()
}
