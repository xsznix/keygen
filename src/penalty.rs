/// Methods for calculating the penalty of a keyboard layout given an input
/// corpus string.

use std::vec::Vec;
use std::ops::Range;
use std::collections::HashMap;
use std::fmt;

use layout::Layout;
use layout::LayoutPosMap;
use layout::KeyMap;
use layout::KeyPress;
use layout::Finger;
use layout::Row;
use layout::KP_NONE;

pub struct KeyPenalty<'a>
{
	name:      &'a str,
	keys_compared: usize,
	f:             Box<Fn(&KeyPress, &Option<KeyPress>, &Option<KeyPress>, &Option<KeyPress>) -> f64>,
}

#[derive(Clone)]
pub struct KeyPenaltyResult<'a>
{
	pub name:  &'a str,
	pub total:     f64,
	pub high_keys: HashMap<&'a str, f64>,
}

pub struct QuartadList<'a>(HashMap<&'a str, usize>);

impl <'a> fmt::Display for KeyPenaltyResult<'a>
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.name, self.total)
	}
}

static BASE_PENALTY: KeyMap<f64> = KeyMap([
	3.0, 1.0, 1.0, 1.0, 3.0,    3.0, 1.0, 1.0, 1.0, 3.0, 4.0,
	0.5, 0.5, 0.0, 0.0, 1.0,    1.0, 0.0, 0.0, 0.5, 0.5, 2.0,
	3.0, 2.5, 2.0, 2.0, 3.0,    3.0, 2.0, 2.0, 2.5, 3.0,
	0.0]);

pub fn init<'a>()
-> Vec<KeyPenalty<'a>>
{
	let mut penalties = Vec::new();

	// Base penalty.
	penalties.push(KeyPenalty {
		name: "base",
		keys_compared: 1,
		f: Box::new(|curr, _, _, _| {
			let KeyMap(base_penalty) = BASE_PENALTY;
			base_penalty[curr.pos]
		}),
	});

	// Penalise 5 points for using the same finger twice on different keys.
	// An extra 5 points for using the centre column.
	penalties.push(KeyPenalty {
		name: "same finger",
		keys_compared: 2,
		f: Box::new(|curr, old, _, _| {
			if let Some(ref old) = *old {
				if curr.hand == old.hand && curr.finger == old.finger && curr.pos != old.pos {
					5.0 + if curr.center { 5.0 } else { 0.0 }
					    + if old.center  { 5.0 } else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		})
	});

	// Penalise 1 point for jumping from top to bottom row or from bottom to
	// top row on the same hand.
	penalties.push(KeyPenalty {
		name: "long jump hand",
		keys_compared: 2,
		f: Box::new(|curr, old, _, _| {
			if let Some(ref old) = *old {
				if curr.hand == old.hand {
					if curr.row == Row::Top && old.row == Row::Bottom ||
					   curr.row == Row::Bottom && old.row == Row::Top {
						1.0
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 10 points for jumping from top to bottom row or from bottom to
	// top row on the same finger.
	penalties.push(KeyPenalty {
		name: "long jump",
		keys_compared: 2,
		f: Box::new(|curr, old, _, _| {
			if let Some(ref old) = *old {
				if curr.hand == old.hand && curr.finger == old.finger {
					if curr.row == Row::Top && old.row == Row::Bottom ||
					   curr.row == Row::Bottom && old.row == Row::Top {
						10.0
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 8 points for jumping from top to bottom row or from bottom to
	// top row on consecutive fingers, except for middle finger-top row ->
	// index finger-bottom row.
	penalties.push(KeyPenalty {
		name: "long jump consecutive",
		keys_compared: 2,
		f: Box::new(|curr, old, _, _| {
			if let Some(ref old) = *old {
				if curr.hand == old.hand &&
						(curr.row == Row::Top && old.row == Row::Bottom ||
						 curr.row == Row::Bottom && old.row == Row::Top) {
					if curr.finger == Finger::Ring   && old.finger == Finger::Pinky  ||
					   curr.finger == Finger::Pinky  && old.finger == Finger::Ring   ||
					   curr.finger == Finger::Middle && old.finger == Finger::Ring   ||
					   curr.finger == Finger::Ring   && old.finger == Finger::Middle ||
					  (curr.finger == Finger::Index  && (old.finger == Finger::Middle ||
					                                     old.finger == Finger::Ring) &&
					   curr.row == Row::Top && old.row == Row::Bottom) {
						8.0
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 10 points for awkward pinky/ring combination where the pinky
	// reaches above the ring finger, e.g. QA/AQ, PL/LP, ZX/XZ, ;./.; on Qwerty.
	penalties.push(KeyPenalty {
		name: "pinky/ring twist",
		keys_compared: 2,
		f: Box::new(|curr, old, _, _| {
			if let Some(ref old) = *old {
				if curr.hand == old.hand {
					if curr.finger == Finger::Ring && old.finger == Finger::Pinky {
						if curr.row == Row::Home && old.row == Row::Top ||
						   curr.row == Row::Bottom && old.row == Row::Home {
							10.0
						} else { 0.0 }
					} else if curr.finger == Finger::Pinky && old.finger == Finger::Ring {
						if curr.row == Row::Top && old.row == Row::Home ||
						   curr.row == Row::Home && old.row == Row::Bottom {
							10.0
						} else { 0.0 }
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 10 points for reversing a roll at the end of the hand, i.e.
	// using the ring, pinky, then middle finger of the same hand, or the
	// middle, pinky, then ring of the same hand.
	penalties.push(KeyPenalty {
		name: "roll reversal",
		keys_compared: 3,
		f: Box::new(|curr, old1, old2, _| {
			if let Some(ref old1) = *old1 {
				if let Some(ref old2) = *old2 {
					if curr.hand == old1.hand && old1.hand == old2.hand {
						if (curr.finger == Finger::Middle && old1.finger == Finger::Pinky && old2.finger == Finger::Ring) ||
						    curr.finger == Finger::Ring && old1.finger == Finger::Pinky && old2.finger == Finger::Middle {
							10.0
						} else { 0.0 }
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 0.5 points for using the same hand four times in a row.
	penalties.push(KeyPenalty {
		name: "same hand",
		keys_compared: 4,
		f: Box::new(|curr, old1, old2, old3| {
			if let Some(ref old1) = *old1 {
				if let Some(ref old2) = *old2 {
					if let Some(ref old3) = *old3 {
						if curr.hand == old1.hand && old1.hand == old2.hand && old2.hand == old3.hand {
							0.5
						} else { 0.0 }
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 0.5 points for alternating hands three times in a row.
	penalties.push(KeyPenalty {
		name: "alternating hand",
		keys_compared: 4,
		f: Box::new(|curr, old1, old2, old3| {
			if let Some(ref old1) = *old1 {
				if let Some(ref old2) = *old2 {
					if let Some(ref old3) = *old3 {
						if curr.hand != old1.hand && old1.hand != old2.hand && old2.hand != old3.hand {
							0.5
						} else { 0.0 }
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Penalise 0.25 points for rolling outwards.
	penalties.push(KeyPenalty {
		name: "roll out",
		keys_compared: 2,
		f: Box::new(|curr, old1, _, _| {
			if let Some(ref old1) = *old1 {
				if curr.hand == old1.hand {
					if old1.finger == Finger::Ring && curr.finger == Finger::Pinky ||
					   old1.finger == Finger::Middle &&
					       (curr.finger == Finger::Ring ||
					        curr.finger == Finger::Pinky) ||
					   old1.finger == Finger::Index &&
					       (curr.finger == Finger::Middle ||
					        curr.finger == Finger::Ring ||
					        curr.finger == Finger::Pinky) {
						0.125
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	// Award 0.25 points for rolling inwards.
	penalties.push(KeyPenalty {
		name: "roll in",
		keys_compared: 2,
		f: Box::new(|curr, old1, _, _| {
			if let Some(ref old1) = *old1 {
				if curr.hand == old1.hand {
					if old1.finger == Finger::Pinky &&
					       (curr.finger == Finger::Ring ||
					        curr.finger == Finger::Middle ||
					        curr.finger == Finger::Index) ||
					   old1.finger == Finger::Ring &&
					       (curr.finger == Finger::Middle ||
					        curr.finger == Finger::Index) ||
					   old1.finger == Finger::Middle && curr.finger == Finger::Index {
						-0.125
					} else { 0.0 }
				} else { 0.0 }
			} else { 0.0 }
		}),
	});

	penalties
}

pub fn prepare_quartad_list<'a>(
	string:       &'a str,
	position_map: &'a LayoutPosMap)
-> QuartadList<'a>
{
	let mut range: Range<usize> = 0..0;
	let mut quartads: HashMap<&str, usize> = HashMap::new();
	for (i, c) in string.chars().enumerate() {
		match *position_map.get_key_position(c) {
			Some(_) => {
				range.end = i + 1;
				if range.end > 3 && range.start < range.end - 4 {
					range.start = range.end - 4;
				}
				let quartad = &string[range.clone()];
				let entry = quartads.entry(quartad).or_insert(0);
				*entry += 1;
			},
			None => {
				range = (i + 1)..(i + 1);
			}
		}
	}

	QuartadList(quartads)
}

pub fn calculate_penalty<'a>(
	quartads:  &   QuartadList<'a>,
	len:           usize,
	layout:    &   Layout,
	penalties: &'a Vec<KeyPenalty>)
-> (f64, f64, Vec<KeyPenaltyResult<'a>>)
{
	let QuartadList(ref quartads) = *quartads;
	let mut result: Vec<KeyPenaltyResult> = Vec::new();
	let mut total = 0.0;
	for penalty in penalties {
		result.push(KeyPenaltyResult {
			name: penalty.name,
			total: 0.0,
			high_keys: HashMap::new(),
		});
	}

	let position_map = layout.get_position_map();
	for (string, count) in quartads {
		let p = penalty_for_quartad(string, *count, &position_map, &penalties, &mut result);
		if p != 0.0 {
			total += p;
		}
	}

	(total, total / (len as f64), result)
}

fn penalty_for_quartad<'a, 'b>(
	string:       &'a str,
	count:            usize,
	position_map: &'b LayoutPosMap,
	penalties:    &'a Vec<KeyPenalty>,
	result:       &'b mut Vec<KeyPenaltyResult<'a>>)
-> f64
{
	let len = string.len();
	let count = count as f64;
	let mut total = 0.0;
	let mut chars = string.chars().into_iter().rev();
	let opt_curr = chars.next();
	let opt_old1 = chars.next();
	let opt_old2 = chars.next();
	let opt_old3 = chars.next();

	let curr = match opt_curr {
		Some(c) => match position_map.get_key_position(c) {
			&Some(ref kp) => kp,
			&None => { return 0.0 }
		},
		None => panic!("unreachable")
	};
	let old1 = match opt_old1 {
		Some(c) => position_map.get_key_position(c),
		None => &KP_NONE
	};
	let old2 = match opt_old2 {
		Some(c) => position_map.get_key_position(c),
		None => &KP_NONE
	};
	let old3 = match opt_old3 {
		Some(c) => position_map.get_key_position(c),
		None => &KP_NONE
	};

	for (i, penalty) in penalties.into_iter().enumerate() {
		let p = (*penalty.f)(&curr, old1, old2, old3) * count;
		if p != 0.0 {
			total += p;
			result[i].total += p;

			let slice = &string[(len - penalty.keys_compared)..len];
			let entry = result[i].high_keys.entry(slice).or_insert(0.0);
			*entry += p;
		}
	}

	total
}