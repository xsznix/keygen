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
	3.0, 1.0, 1.0, 1.5, 3.0,    3.0, 1.5, 1.0, 1.0, 3.0, 4.0,
	0.5, 0.5, 0.0, 0.0, 1.5,    1.5, 0.0, 0.0, 0.5, 0.5, 2.0,
	2.0, 2.0, 1.5, 1.5, 2.5,    2.5, 1.5, 1.5, 2.0, 2.0,
	                    0.0,    0.0]);

pub fn init<'a>()
-> Vec<KeyPenalty<'a>>
{
	let mut penalties = Vec::new();

	// Base penalty.
	penalties.push(KeyPenalty {
		name: "base",
	});

	// Penalise 5 points for using the same finger twice on different keys.
	// An extra 5 points for using the centre column.
	penalties.push(KeyPenalty {
		name: "same finger",
	});

	// Penalise 1 point for jumping from top to bottom row or from bottom to
	// top row on the same hand.
	penalties.push(KeyPenalty {
		name: "long jump hand",
	});

	// Penalise 10 points for jumping from top to bottom row or from bottom to
	// top row on the same finger.
	penalties.push(KeyPenalty {
		name: "long jump",
	});

	// Penalise 5 points for jumping from top to bottom row or from bottom to
	// top row on consecutive fingers, except for middle finger-top row ->
	// index finger-bottom row.
	penalties.push(KeyPenalty {
		name: "long jump consecutive",
	});

	// Penalise 10 points for awkward pinky/ring combination where the pinky
	// reaches above the ring finger, e.g. QA/AQ, PL/LP, ZX/XZ, ;./.; on Qwerty.
	penalties.push(KeyPenalty {
		name: "pinky/ring twist",
	});

	// Penalise 20 points for reversing a roll at the end of the hand, i.e.
	// using the ring, pinky, then middle finger of the same hand, or the
	// middle, pinky, then ring of the same hand.
	penalties.push(KeyPenalty {
		name: "roll reversal",
	});

	// Penalise 0.5 points for using the same hand four times in a row.
	penalties.push(KeyPenalty {
		name: "same hand",
	});

	// Penalise 0.5 points for alternating hands three times in a row.
	penalties.push(KeyPenalty {
		name: "alternating hand",
	});

	// Penalise 0.125 points for rolling outwards.
	penalties.push(KeyPenalty {
		name: "roll out",
	});

	// Award 0.125 points for rolling inwards.
	penalties.push(KeyPenalty {
		name: "roll in",
	});

	// Penalise 3 points for jumping from top to bottom row or from bottom to
	// top row on the same finger with a keystroke in between.
	penalties.push(KeyPenalty {
		name: "long jump sandwich",
	});

	// Penalise 10 points for three consecutive keystrokes going up or down the
	// three rows of the keyboard in a roll.
	penalties.push(KeyPenalty {
		name: "twist",
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
	penalties: &'a Vec<KeyPenalty>,
	detailed:      bool)
-> (f64, f64, Vec<KeyPenaltyResult<'a>>)
{
	let QuartadList(ref quartads) = *quartads;
	let mut result: Vec<KeyPenaltyResult> = Vec::new();
	let mut total = 0.0;

	if detailed {
		for penalty in penalties {
			result.push(KeyPenaltyResult {
				name: penalty.name,
				total: 0.0,
				high_keys: HashMap::new(),
			});
		}
	}

	let position_map = layout.get_position_map();
	for (string, count) in quartads {
		total += penalty_for_quartad(string, *count, &position_map, &mut result, detailed);
	}

	(total, total / (len as f64), result)
}

fn penalty_for_quartad<'a, 'b>(
	string:       &'a str,
	count:            usize,
	position_map: &'b LayoutPosMap,
	result:       &'b mut Vec<KeyPenaltyResult<'a>>,
	detailed:         bool)
-> f64
{
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

	penalize(string, count, &curr, old1, old2, old3, result, detailed)
}

fn penalize<'a, 'b>(
	string: &'a     str,
	count:          usize,
	curr:   &              KeyPress,
	old1:   &       Option<KeyPress>,
	old2:   &       Option<KeyPress>,
	old3:   &       Option<KeyPress>,
	result: &'b mut Vec<KeyPenaltyResult<'a>>,
	detailed:       bool)
-> f64
{
	let len = string.len();
	let count = count as f64;
	let mut total = 0.0;

	// One key penalties.
	let slice1 = &string[(len - 1)..len];

	// 0: Base penalty.
	let base = BASE_PENALTY.0[curr.pos] * count;
	if detailed {
		*result[0].high_keys.entry(slice1).or_insert(0.0) += base;
		result[0].total += base;
	}
	total += base;

	// Two key penalties.
	let old1 = match *old1 {
		Some(ref o) => o,
		None => { return total }
	};

	if curr.hand == old1.hand {
		let slice2 = &string[(len - 2)..len];

		// 1: Same finger.
		if curr.finger == old1.finger && curr.pos != old1.pos {
			let penalty = 5.0 + if curr.center { 5.0 } else { 0.0 }
			                  + if old1.center { 5.0 } else { 0.0 };
			let penalty = penalty * count;
			if detailed {
				*result[1].high_keys.entry(slice2).or_insert(0.0) += penalty;
				result[1].total += penalty;
			}
			total += penalty;
		}

		// 2: Long jump hand.
		if curr.row == Row::Top && old1.row == Row::Bottom ||
		   curr.row == Row::Bottom && old1.row == Row::Top {
			let penalty = count;
			if detailed {
				*result[2].high_keys.entry(slice2).or_insert(0.0) += penalty;
				result[2].total += penalty;
			}
			total += penalty;
		}

		// 3: Long jump.
		if curr.hand == old1.hand && curr.finger == old1.finger {
			if curr.row == Row::Top && old1.row == Row::Bottom ||
			   curr.row == Row::Bottom && old1.row == Row::Top {
				let penalty = 10.0 * count;
				if detailed {
					*result[3].high_keys.entry(slice2).or_insert(0.0) += penalty;
					result[3].total += penalty;
				}
				total += penalty;
			}
		}

		// 4: Long jump consecutive.
		if curr.row == Row::Top && old1.row == Row::Bottom ||
		   curr.row == Row::Bottom && old1.row == Row::Top {
			if curr.finger == Finger::Ring   && old1.finger == Finger::Pinky  ||
			   curr.finger == Finger::Pinky  && old1.finger == Finger::Ring   ||
			   curr.finger == Finger::Middle && old1.finger == Finger::Ring   ||
			   curr.finger == Finger::Ring   && old1.finger == Finger::Middle ||
			  (curr.finger == Finger::Index  && (old1.finger == Finger::Middle ||
			                                     old1.finger == Finger::Ring) &&
			   curr.row == Row::Top && old1.row == Row::Bottom) {
				let penalty = 5.0 * count;
				if detailed {
					*result[4].high_keys.entry(slice2).or_insert(0.0) += penalty;
					result[4].total += penalty;
				}
				total += penalty;
			}
		}

		// 5: Pinky/ring twist.
		if (curr.finger == Finger::Ring && old1.finger == Finger::Pinky &&
		    (curr.row == Row::Home && old1.row == Row::Top ||
		     curr.row == Row::Bottom && old1.row == Row::Top)) ||
		   (curr.finger == Finger::Pinky && old1.finger == Finger::Ring &&
		    (curr.row == Row::Top && old1.row == Row::Home ||
		     curr.row == Row::Top && old1.row == Row::Bottom)) {
			let penalty = 10.0 * count;
			if detailed {
				*result[5].high_keys.entry(slice2).or_insert(0.0) += penalty;
				result[5].total += penalty;
			}
			total += penalty;
		}

		// 9: Roll out.
		if curr.hand == old1.hand &&
		   old1.finger != Finger::Thumb &&
		   is_roll_out(curr.finger, old1.finger) {
			let penalty = 0.125 * count;
			if detailed {
				*result[9].high_keys.entry(slice2).or_insert(0.0) += penalty;
				result[9].total += penalty;
			}
			total += penalty;
		}

		// 10: Roll in.
		if curr.hand == old1.hand && is_roll_in(curr.finger, old1.finger) {
			let penalty = -0.125 * count;
			if detailed {
				*result[10].high_keys.entry(slice2).or_insert(0.0) += penalty;
				result[10].total += penalty;
			}
			total += penalty;
		}
	}

	// Three key penalties.
	let old2 = match *old2 {
		Some(ref o) => o,
		None => { return total },
	};

	if curr.hand == old1.hand && old1.hand == old2.hand {
		// 6: Roll reversal.
		if (curr.finger == Finger::Middle && old1.finger == Finger::Pinky && old2.finger == Finger::Ring) ||
		    curr.finger == Finger::Ring && old1.finger == Finger::Pinky && old2.finger == Finger::Middle {
			let slice3 = &string[(len - 3)..len];
			let penalty = 20.0 * count;
			if detailed {
				*result[6].high_keys.entry(slice3).or_insert(0.0) += penalty;
				result[6].total += penalty;
			}
			total += penalty;
		}

		// 12: Twist.
		if ((curr.row == Row::Top && old1.row == Row::Home && old2.row == Row::Bottom) ||
		    (curr.row == Row::Bottom && old1.row == Row::Home && old2.row == Row::Top)) &&
		   ((is_roll_out(curr.finger, old1.finger) && is_roll_out(old1.finger, old2.finger)) ||
		   	(is_roll_in(curr.finger, old1.finger) && is_roll_in(old1.finger, old2.finger))) {
			let slice3 = &string[(len - 3)..len];
			let penalty = 10.0 * count;
			if detailed {
				*result[12].high_keys.entry(slice3).or_insert(0.0) += penalty;
				result[12].total += penalty;
			}
			total += penalty;
		}
	}

	// 11: Long jump sandwich.
	if curr.hand == old2.hand && curr.finger == old2.finger {
		if curr.row == Row::Top && old2.row == Row::Bottom ||
		   curr.row == Row::Bottom && old2.row == Row::Top {
			let penalty = 3.0 * count;
			if detailed {
				let slice3 = &string[(len - 3)..len];
				*result[11].high_keys.entry(slice3).or_insert(0.0) += penalty;
				result[11].total += penalty;
			}
			total += penalty;
		}
	}

	// Four key penalties.
	let old3 = match *old3 {
		Some(ref o) => o,
		None => { return total },
	};

	if curr.hand == old1.hand && old1.hand == old2.hand && old2.hand == old3.hand {
		// 7: Same hand.
		let slice4 = &string[(len - 4)..len];
		let penalty = 0.5 * count;
		if detailed {
			*result[7].high_keys.entry(slice4).or_insert(0.0) += penalty;
			result[7].total += penalty;
		}
		total += penalty;
	} else if curr.hand != old1.hand && old1.hand != old2.hand && old2.hand != old3.hand {
		// 8: Alternating hand.
		let slice4 = &string[(len - 4)..len];
		let penalty = 0.5 * count;
		if detailed {
			*result[8].high_keys.entry(slice4).or_insert(0.0) += penalty;
			result[8].total += penalty;
		}
		total += penalty;
	}

	total
}

fn is_roll_out(curr: Finger, prev: Finger) -> bool {
	match curr {
		Finger::Thumb  => false,
		Finger::Index  => prev == Finger::Thumb,
		Finger::Middle => prev == Finger::Thumb || prev == Finger::Index,
		Finger::Ring   => prev != Finger::Pinky && prev != Finger::Ring,
		Finger::Pinky  => prev != Finger::Pinky
	}
}

fn is_roll_in(curr: Finger, prev: Finger) -> bool {
	match curr {
		Finger::Thumb  => prev != Finger::Thumb,
		Finger::Index  => prev != Finger::Thumb && prev != Finger::Index,
		Finger::Middle => prev == Finger::Pinky || prev == Finger::Ring,
		Finger::Ring   => prev == Finger::Pinky,
		Finger::Pinky  => false,
	}
}
