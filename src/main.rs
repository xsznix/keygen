#![feature(linked_list_extras)]

mod layout;
mod penalty;
mod annealing;
mod simulator;

use std::env;
use std::fs::File;
use std::io::Read;

fn main()
{
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

			let init_pos_map = layout::INIT_LAYOUT.get_position_map();
			let quartads = penalty::prepare_quartad_list(&s[..], &init_pos_map);
			let len = s.len();

			// // Comment out the following...
			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::QWERTY_LAYOUT, &penalties);
			// println!("Reference: QWERTY");
			// simulator::print_result(&layout::QWERTY_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::DVORAK_LAYOUT, &penalties);
			// println!("Reference: DVORAK");
			// simulator::print_result(&layout::DVORAK_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::COLEMAK_LAYOUT, &penalties);
			// println!("Reference: COLEMAK");
			// simulator::print_result(&layout::COLEMAK_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::QGMLWY_LAYOUT, &penalties);
			// println!("Reference: QGMLWY");
			// simulator::print_result(&layout::QGMLWY_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::WORKMAN_LAYOUT, &penalties);
			// println!("Reference: WORKMAN");
			// simulator::print_result(&layout::WORKMAN_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::MALTRON_LAYOUT, &penalties);
			// println!("Reference: MALTRON");
			// simulator::print_result(&layout::MALTRON_LAYOUT, &penalty);
			// println!("");

			// let penalty = penalty::calculate_penalty(&quartads, len, &layout::INIT_LAYOUT, &penalties);
			// println!("Reference: INITIAL");
			// simulator::print_result(&layout::INIT_LAYOUT, &penalty);
			// // ...to skip the reference calculations.

			for _ in 0..10000 {
				simulator::simulate(&quartads, len, &layout::INIT_LAYOUT, &penalties);
			}
		} else {
			panic!("Could not open corpus.");
		}
	}
}