/// Applies the math in annealing.rs to keyboard layouts.


extern crate rand;

use self::rand::random;
use std::cmp::Ordering;
use std::collections::LinkedList;

use layout;
use penalty;
use annealing;

const MAX_SWAPS_PER_ITERATION: usize = 3;
const MAX_BEST_LAYOUTS: usize = 1;

struct BestLayoutsEntry<'a>
{
	layout:         layout::Layout,
	total_penalty:  f64,
	scaled_penalty: f64,
	penalties:      Vec<penalty::KeyPenaltyResult<'a>>,
}

impl <'a> BestLayoutsEntry<'a>
{
	fn cmp(&self, other: &BestLayoutsEntry)
	-> Ordering
	{
		match self.scaled_penalty.partial_cmp(&other.scaled_penalty) {
			Some(ord) => ord,
			None => Ordering::Equal
		}
	}
}

pub fn simulate<'a>(
	quartads:    &penalty::QuartadList<'a>,
	len:          usize,
	init_layout: &layout::Layout,
	penalties:   &Vec<penalty::KeyPenalty<'a>>)
{
	let penalty = penalty::calculate_penalty(&quartads, len, init_layout, penalties);
	// println!("Initial layout:");
	// print_result(init_layout, &penalty);

	// Keep track of the best layouts we've encountered.
	let mut best_layouts: LinkedList<BestLayoutsEntry> = LinkedList::new();

	let mut accepted_layout = init_layout.clone();
	let mut accepted_penalty = penalty.1;
	for i in annealing::get_simulation_range() {
		// Copy and shuffle this iteration of the layout.
		let mut curr_layout = accepted_layout.clone();
		curr_layout.shuffle(random::<usize>() % MAX_SWAPS_PER_ITERATION + 1);

		// Calculate penalty.
		let curr_layout_copy = curr_layout.clone();
		let penalty = penalty::calculate_penalty(&quartads, len, &curr_layout, penalties);
		let scaled_penalty = penalty.1;

		// Probabilistically accept worse transitions; always accept better
		// transitions.
		if annealing::accept_transition(scaled_penalty - accepted_penalty, i) {
			// println!("Iteration {} accepted with penalty {}", i, scaled_penalty);

			accepted_layout = curr_layout_copy.clone();
			accepted_penalty = scaled_penalty;

			// New entry to insert into best layouts.
			let new_entry = BestLayoutsEntry {
				layout: curr_layout_copy,
				total_penalty: penalty.0,
				scaled_penalty: penalty.1,
				penalties: penalty.2
			};

			{
				// Find where to add our new entry to, since the list is sorted.
				let mut iter = best_layouts.iter_mut();
				loop {
					{
						let opt_next = iter.peek_next();
						if let Some(next) = opt_next {
							let cmp = new_entry.cmp(next);
							if cmp == Ordering::Less {
								break;
							}
						} else {
							break;
						}
					}

					iter.next();
				}

				// Add to list.
				iter.insert_next(new_entry);
			}

			// Limit best layouts list to ten items.
			while best_layouts.len() > MAX_BEST_LAYOUTS {
				best_layouts.pop_back();
			}
		}
	}

	for entry in best_layouts.into_iter() {
		let layout = entry.layout;
		let penalty = (entry.total_penalty, entry.scaled_penalty, entry.penalties);
		println!("");
		print_result(&layout, &penalty);
	}
}

pub fn print_result<'a>(
	layout: &'a layout::Layout,
	penalty: &'a (f64, f64, Vec<penalty::KeyPenaltyResult<'a>>))
{
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
		for key in high_keys.iter().take(5) {
			let (k, v) = *key;
			print!(" {}: {};", k, v);
		}
		println!("");
	}
}