/// Applies the math in annealing.rs to keyboard layouts.

extern crate rand;

use self::rand::random;
use std::cmp::Ordering;

use layout;
use penalty;
use annealing;

const MAX_SWAPS_PER_ITERATION: usize = 4;

pub fn simulate<'a>(string: &'a str, init_layout: &'a layout::Layout, penalties: &'a Vec<penalty::KeyPenalty<'a>>) {
	let penalty = penalty::calculate_penalty(string, init_layout, penalties);
	println!("Initial layout:");
	print_result(init_layout, &penalty);

	let mut accepted_layout = init_layout.clone();
	let mut accepted_penalty = penalty.1;
	for i in annealing::get_simulation_range() {
		let mut curr_layout = accepted_layout.clone();
		let curr_penalty;
		curr_layout.shuffle(random::<usize>() % MAX_SWAPS_PER_ITERATION + 1);
		{
			let penalty = penalty::calculate_penalty(string, &curr_layout, penalties);
			println!("Iteration {}", i);
			print_result(&curr_layout, &penalty);
			curr_penalty = penalty.1;
		}
		if annealing::accept_transition(curr_penalty - accepted_penalty, i) {
			accepted_layout = curr_layout;
			accepted_penalty = curr_penalty;
			println!("ACCEPTED");
		} else {
			println!("DENIED");
		}
		println!("");
	}
}

pub fn print_result<'a>(layout: &'a layout::Layout, penalty: &'a (f64, f64, Vec<penalty::KeyPenaltyResult<'a>>)) {
	println!("{}", layout);

	let (ref total, ref scaled, ref penalties) = *penalty;
	println!("total: {}; scaled: {}", total, scaled);
	for penalty in penalties {
		print!("{}  / ", penalty);
		let mut high_keys: Vec<(&str, f64)> = penalty.high_keys.iter().map(|x| (*x.0, *x.1)).collect();
		high_keys.sort_by(|a, b|
			match b.1.abs().partial_cmp(&a.1.abs()) {
				Some(c) => c,
				None => Ordering::Equal
			});
		for key in high_keys.iter().take(3) {
			let (k, v) = *key;
			print!(" {}: {};", k, v);
		}
		println!("");
	}
}