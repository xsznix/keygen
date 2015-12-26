mod layout;
mod penalty;
mod annealing;
mod simulator;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
	let penalties = penalty::init();
	if let Some(test_str) = env::args().nth(1) {
		if let Ok(mut f) = File::open(test_str) {
			let mut s = String::new();
			match f.read_to_string(&mut s) {
				Err(e) => {
					println!("Error: {}", e);
					panic!("could not read corpus");
				},
				_ => (),
			}

			// Comment out the following...
			let penalty = penalty::calculate_penalty(&s[..], &layout::QWERTY_LAYOUT, &penalties);
			println!("Reference: QWERTY");
			simulator::print_result(&layout::QWERTY_LAYOUT, &penalty);
			println!("");

			let penalty = penalty::calculate_penalty(&s[..], &layout::DVORAK_LAYOUT, &penalties);
			println!("Reference: DVORAK");
			simulator::print_result(&layout::DVORAK_LAYOUT, &penalty);
			println!("");

			let penalty = penalty::calculate_penalty(&s[..], &layout::COLEMAK_LAYOUT, &penalties);
			println!("Reference: COLEMAK");
			simulator::print_result(&layout::COLEMAK_LAYOUT, &penalty);
			println!("");

			let penalty = penalty::calculate_penalty(&s[..], &layout::QGMLWY_LAYOUT, &penalties);
			println!("Reference: QGMLWY");
			simulator::print_result(&layout::QGMLWY_LAYOUT, &penalty);
			println!("");

			let penalty = penalty::calculate_penalty(&s[..], &layout::WORKMAN_LAYOUT, &penalties);
			println!("Reference: WORKMAN");
			simulator::print_result(&layout::WORKMAN_LAYOUT, &penalty);
			println!("");
			// ...to skip the reference calculations.

			simulator::simulate(&s[..], &layout::INIT_LAYOUT, &penalties);
		} else {
			panic!("Could not open corpus.");
		}
	}
}