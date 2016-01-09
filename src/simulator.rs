/// Applies the math in annealing.rs to keyboard layouts.


extern crate rand;

use self::rand::random;
use std::cmp::Ordering;
use std::collections::LinkedList;

use layout;
use penalty;
use annealing;

struct BestLayoutsEntry
{
	layout:  layout::Layout,
	penalty: f64,
}

impl BestLayoutsEntry
{
	fn cmp(&self, other: &BestLayoutsEntry)
	-> Ordering
	{
		match self.penalty.partial_cmp(&other.penalty) {
			Some(ord) => ord,
			None => Ordering::Equal
		}
	}
}

pub fn simulate<'a>(
	quartads:    &penalty::QuartadList<'a>,
	len:          usize,
	init_layout: &layout::Layout,
	penalties:   &Vec<penalty::KeyPenalty<'a>>,
	debug:        bool,
	top_layouts:  usize,
	num_swaps:    usize)
{
	let penalty = penalty::calculate_penalty(&quartads, len, init_layout, penalties, true);

	if debug {
		println!("Initial layout:");
		print_result(init_layout, &penalty);
	}

	// Keep track of the best layouts we've encountered.
	let mut best_layouts: LinkedList<BestLayoutsEntry> = LinkedList::new();

	let mut accepted_layout = init_layout.clone();
	let mut accepted_penalty = penalty.1;
	for i in annealing::get_simulation_range() {
		// Copy and shuffle this iteration of the layout.
		let mut curr_layout = accepted_layout.clone();
		curr_layout.shuffle(random::<usize>() % num_swaps + 1);

		// Calculate penalty.
		let curr_layout_copy = curr_layout.clone();
		let penalty = penalty::calculate_penalty(&quartads, len, &curr_layout, penalties, false);
		let scaled_penalty = penalty.1;

		// Probabilistically accept worse transitions; always accept better
		// transitions.
		if annealing::accept_transition(scaled_penalty - accepted_penalty, i) {
			if debug {
				println!("Iteration {} accepted with penalty {}", i, scaled_penalty);
			}

			accepted_layout = curr_layout_copy.clone();
			accepted_penalty = scaled_penalty;

			// Insert this layout into best layouts.
			let new_entry = BestLayoutsEntry {
				layout: curr_layout_copy,
				penalty: penalty.1,
			};
			best_layouts = list_insert_ordered(best_layouts, new_entry);

			// Limit best layouts list length.
			while best_layouts.len() > top_layouts {
				best_layouts.pop_back();
			}
		}
	}

	for entry in best_layouts.into_iter() {
		let layout = entry.layout;
		let penalty = penalty::calculate_penalty(&quartads, len, &layout, penalties, true);
		println!("");
		print_result(&layout, &penalty);
	}
}

pub fn refine<'a>(
	quartads:    &penalty::QuartadList<'a>,
	len:          usize,
	init_layout: &layout::Layout,
	penalties:   &Vec<penalty::KeyPenalty<'a>>,
	debug:        bool,
	top_layouts:  usize,
	num_swaps:    usize)
{
	let penalty = penalty::calculate_penalty(&quartads, len, init_layout, penalties, true);

	println!("Initial layout:");
	print_result(init_layout, &penalty);

	let mut curr_layout = init_layout.clone();
	let mut curr_penalty = penalty.1;

	loop {
		// Test every layout within `num_swaps` swaps of the initial layout.
		let mut best_layouts: LinkedList<BestLayoutsEntry> = LinkedList::new();
		let permutations = layout::LayoutPermutations::new(init_layout, num_swaps);
		for (i, layout) in permutations.enumerate() {
			let penalty = penalty::calculate_penalty(&quartads, len, &layout, penalties, false);

			if debug {
				println!("Iteration {}: {}", i, penalty.1);
			}

			// Insert this layout into best layouts.
			let new_entry = BestLayoutsEntry {
				layout: layout,
				penalty: penalty.1,
			};
			best_layouts = list_insert_ordered(best_layouts, new_entry);

			// Limit best layouts list length.
			while best_layouts.len() > top_layouts {
				best_layouts.pop_back();
			}
		}

		// Print the top layouts.
		for entry in best_layouts.iter() {
			let ref layout = entry.layout;
			let penalty = penalty::calculate_penalty(&quartads, len, &layout, penalties, true);
			println!("");
			print_result(&layout, &penalty);
		}

		// Keep going until swapping doesn't get us any more improvements.
		let best = best_layouts.pop_front().unwrap();
		if curr_penalty <= best.penalty {
			break;
		} else {
			curr_layout = best.layout;
			curr_penalty = best.penalty;
		}
	}

	println!("");
	println!("Ultimate winner:");
	println!("{}", curr_layout);
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

// Take ownership of the list and give it back as a hack to make the borrow checker happy :^)
fn list_insert_ordered(mut list: LinkedList<BestLayoutsEntry>, entry: BestLayoutsEntry)
-> LinkedList<BestLayoutsEntry>
{
	{
		// Find where to add our new entry to, since the list is sorted.
		let mut iter = list.iter_mut();
		loop {
			{
				let opt_next = iter.peek_next();
				if let Some(next) = opt_next {
					let cmp = entry.cmp(next);
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
		iter.insert_next(entry);
	}
	list
}