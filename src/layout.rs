/// Data structures and methods for creating and shuffling keyboard layouts.

extern crate rand;

use std::fmt;
use self::rand::random;

/* ----- *
 * TYPES *
 * ----- */

// KeyMap format:
//    LEFT HAND   |    RIGHT HAND
//  0  1  2  3  4 |  5  6  7  8  9 10
// 11 12 13 14 15 | 16 17 18 19 20 21 
// 22 23 24 25 26 | 27 28 29 30 31
//
//             32 (thumb key)

pub struct KeyMap<T>(pub [T; 33]);

impl <T: Copy> Clone for KeyMap<T>
{
	fn clone(&self)
	-> KeyMap<T>
	{
		KeyMap(self.0)
	}
}

#[derive(Clone)]
pub struct Layer(KeyMap<char>);

#[derive(Clone)]
pub struct Layout(Layer, Layer);

pub struct LayoutPosMap([Option<KeyPress>; 128]);

#[derive(Clone)]
pub struct LayoutShuffleMask(KeyMap<bool>);

#[derive(Clone, Copy, PartialEq)]
pub enum Finger 
{
	Thumb,
	Index,
	Middle,
	Ring,
	Pinky,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Hand
{
	Left,
	Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Row
{
	Top,
	Home,
	Bottom,
	Thumb,
}

#[derive(Clone, Copy)]
pub struct KeyPress
{
	pub kc:     char,
	pub pos:    usize,
	pub finger: Finger,
	pub hand:   Hand,
	pub row:    Row,
}

/* ------- *
 * STATICS *
 * ------- */

pub static INIT_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'u', 'p', 'g', '/',   'z', 'l', 'w', 'y', '\'', '=',
	              'a', 'r', 'n', 's', 'd',   'f', 'h', 't', 'i', 'o',  '-',
	              'j', 'k', 'v', 'c', ';',   'x', 'm', 'b', ',', '.',
	              'e'])),
	Layer(KeyMap(['Q', 'U', 'P', 'G', '?',   'Z', 'L', 'W', 'Y', '"', '+',
	              'A', 'R', 'N', 'S', 'D',   'F', 'H', 'T', 'I', 'O', '_',
	              'J', 'K', 'V', 'C', ':',   'X', 'M', 'B', '<', '>',
	              'E'])));

#[allow(dead_code)]
pub static QWERTY_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'w', 'e', 'r', 't',   'y', 'u', 'i', 'o', 'p', '-',
	              'a', 's', 'd', 'f', 'g',   'h', 'j', 'k', 'l', ';', '\'',
	              'z', 'x', 'c', 'v', 'b',   'n', 'm', ',', '.', '/',
	              '\0'])),
	Layer(KeyMap(['Q', 'W', 'E', 'R', 'T',   'Y', 'U', 'I', 'O', 'P', '_',
	              'A', 'S', 'D', 'F', 'G',   'H', 'J', 'K', 'L', ':', '"',
	              'Z', 'X', 'C', 'V', 'B',   'N', 'M', '<', '>', '?',
	              '\0'])));

#[allow(dead_code)]
pub static DVORAK_LAYOUT: Layout = Layout(
	Layer(KeyMap(['\'', ',', '.', 'p', 'y',   'f', 'g', 'c', 'r', 'l', '/',
	              'a', 'o', 'e', 'u', 'i',   'd', 'h', 't', 'n', 's', '-',
	              ';', 'q', 'j', 'k', 'x',   'b', 'm', 'w', 'v', 'z',
	              '\0'])),
	Layer(KeyMap(['"', ',', '.', 'P', 'Y',   'F', 'G', 'C', 'R', 'L', '?',
	              'A', 'O', 'E', 'U', 'I',   'D', 'H', 'T', 'N', 'S', '_',
	              ':', 'Q', 'J', 'K', 'X',   'B', 'M', 'W', 'V', 'Z',
	              '\0'])));

#[allow(dead_code)]
pub static COLEMAK_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'w', 'f', 'p', 'g',   'j', 'l', 'u', 'y', ';', '-',
	              'a', 'r', 's', 't', 'd',   'h', 'n', 'e', 'i', 'o', '\'',
	              'z', 'x', 'c', 'v', 'b',   'k', 'm', ',', '.', '/',
	              '\0'])),
	Layer(KeyMap(['Q', 'W', 'F', 'P', 'G',   'J', 'L', 'U', 'Y', ':', '_',
	              'A', 'R', 'S', 'T', 'D',   'H', 'N', 'E', 'I', 'O', '"',
	              'Z', 'X', 'C', 'V', 'B',   'K', 'M', '<', '>', 'Z',
	              '\0'])));

#[allow(dead_code)]
pub static QGMLWY_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'g', 'm', 'l', 'w',   'y', 'f', 'u', 'b', ';', '-',
	              'd', 's', 't', 'n', 'r',   'i', 'a', 'e', 'o', 'h', '\'',
	              'z', 'x', 'c', 'v', 'j',   'k', 'p', ',', '.', '/',
	              '\0'])),
	Layer(KeyMap(['Q', 'G', 'M', 'L', 'W',   'Y', 'F', 'U', 'B', ';', '-',
	              'D', 'S', 'T', 'N', 'R',   'I', 'A', 'E', 'O', 'H', '\'',
	              'Z', 'X', 'C', 'V', 'J',   'K', 'P', ',', '.', '/',
	              '\0'])));

#[allow(dead_code)]
pub static WORKMAN_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'd', 'r', 'w', 'b',   'j', 'f', 'u', 'p', ';', '-',
	              'a', 's', 'h', 't', 'g',   'y', 'n', 'e', 'o', 'i', '\'',
	              'z', 'x', 'm', 'c', 'v',   'k', 'l', ',', '.', '/',
	              '\0'])),
	Layer(KeyMap(['Q', 'D', 'R', 'W', 'B',   'J', 'F', 'U', 'P', ';', '-',
	              'A', 'S', 'H', 'T', 'G',   'Y', 'N', 'E', 'O', 'I', '\'',
	              'Z', 'X', 'M', 'C', 'V',   'K', 'L', ',', '.', '/',
	              '\0'])));

#[allow(dead_code)]
pub static MALTRON_LAYOUT: Layout = Layout(
	Layer(KeyMap(['q', 'p', 'y', 'c', 'b',   'v', 'm', 'u', 'z', 'l', '=',
	              'a', 'n', 'i', 's', 'f',   'd', 't', 'h', 'o', 'r', '\'',
	              ',', '.', 'j', 'g', '/',   ';', 'w', 'k', '-', 'x',
	              'e'])),
	Layer(KeyMap(['Q', 'P', 'Y', 'C', 'B',   'V', 'M', 'U', 'Z', 'L', '+',
	              'A', 'N', 'I', 'S', 'F',   'D', 'T', 'H', 'O', 'R', '"',
	              '<', '>', 'J', 'G', '?',   ':', 'W', 'K', '_', 'X',
	              'E'])));

static LAYOUT_MASK: LayoutShuffleMask = LayoutShuffleMask(KeyMap([
	true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false,
	true,  true,  true,  true,  true,  true,  true,  true,  true,  true,  false,
	true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
	false]));
static LAYOUT_MASK_NUM_SWAPPABLE: usize = 30;

static KEY_FINGERS: KeyMap<Finger> = KeyMap([
	Finger::Pinky, Finger::Ring, Finger::Middle, Finger::Index, Finger::Index,    Finger::Index, Finger::Index, Finger::Middle, Finger::Ring, Finger::Pinky, Finger::Pinky,
	Finger::Pinky, Finger::Ring, Finger::Middle, Finger::Index, Finger::Index,    Finger::Index, Finger::Index, Finger::Middle, Finger::Ring, Finger::Pinky, Finger::Pinky,
	Finger::Pinky, Finger::Ring, Finger::Middle, Finger::Index, Finger::Index,    Finger::Index, Finger::Index, Finger::Middle, Finger::Ring, Finger::Pinky,
	Finger::Thumb]);
static KEY_HANDS: KeyMap<Hand> = KeyMap([
	Hand::Left, Hand::Left, Hand::Left, Hand::Left, Hand::Left,    Hand::Right, Hand::Right, Hand::Right, Hand::Right, Hand::Right, Hand::Right,
	Hand::Left, Hand::Left, Hand::Left, Hand::Left, Hand::Left,    Hand::Right, Hand::Right, Hand::Right, Hand::Right, Hand::Right, Hand::Right,
	Hand::Left, Hand::Left, Hand::Left, Hand::Left, Hand::Left,    Hand::Right, Hand::Right, Hand::Right, Hand::Right, Hand::Right,
	Hand::Left]);
static KEY_ROWS: KeyMap<Row> = KeyMap([
	Row::Top,    Row::Top,    Row::Top,    Row::Top,    Row::Top,       Row::Top,    Row::Top,    Row::Top,    Row::Top,    Row::Top,    Row::Top,
	Row::Home,   Row::Home,   Row::Home,   Row::Home,   Row::Home,      Row::Home,   Row::Home,   Row::Home,   Row::Home,   Row::Home,   Row::Home,
	Row::Bottom, Row::Bottom, Row::Bottom, Row::Bottom, Row::Bottom,    Row::Bottom, Row::Bottom, Row::Bottom, Row::Bottom, Row::Bottom,
	Row::Thumb]);

pub static KP_NONE: Option<KeyPress> = None;

/* ----- *
 * IMPLS *
 * ----- */

impl Layout
{
	pub fn shuffle(&mut self, times: usize)
	{
		for _ in 0..times {
			let (i, j) = Layout::shuffle_position();
			let Layout(ref mut lower, ref mut upper) = *self;
			lower.swap(i, j);
			upper.swap(i, j);
		}
	}

	pub fn get_position_map(&self)
	-> LayoutPosMap
	{
		let Layout(ref lower, ref upper) = *self;
		let mut map = [None; 128];
		lower.fill_position_map(&mut map);
		upper.fill_position_map(&mut map);

		LayoutPosMap(map)
	}

	fn shuffle_position() 
	-> (usize, usize)
	{
		let LayoutShuffleMask(KeyMap(ref mask)) = LAYOUT_MASK;
		let mut i = random::<usize>() % LAYOUT_MASK_NUM_SWAPPABLE;
		let mut j = random::<usize>() % (LAYOUT_MASK_NUM_SWAPPABLE - 1);
		if j >= i {
			j += 1;
		}
		// println!("i j = {} {}", i, j);

		let mut k = 0;
		while k <= i {
			if mask[k] == false {
				i += 1;
			}
			k += 1;
		}

		k = 0;
		while k <= j {
			if mask[k] == false {
				j += 1;
			}
			k += 1;
		}
		(i, j)
	}
}

impl Layer
{
	fn swap(&mut self, i: usize, j: usize)
	{
		let Layer(KeyMap(ref mut layer)) = *self;
		let temp = layer[i];
		layer[i] = layer[j];
		layer[j] = temp;
	}

	fn fill_position_map(&self, map: &mut [Option<KeyPress>; 128])
	{
		let Layer(KeyMap(ref layer)) = *self;
		let KeyMap(ref fingers) = KEY_FINGERS;
		let KeyMap(ref hands) = KEY_HANDS;
		let KeyMap(ref rows) = KEY_ROWS;
		for (i, c) in layer.into_iter().enumerate() {
			if *c < (128 as char) {
				map[*c as usize] = Some(KeyPress {
					kc: *c,
					pos: i,
					finger: fingers[i],
					hand: hands[i],
					row: rows[i],
				});
			}
		}
	}
}

impl LayoutPosMap
{
	pub fn get_key_position(&self, kc: char)
	-> &Option<KeyPress>
	{
		let LayoutPosMap(ref map) = *self;
		if kc < (128 as char) {
			&map[kc as usize]
		} else {
			&KP_NONE
		}
	}
}

impl fmt::Display for Layout
{
	fn fmt(&self, f: &mut fmt::Formatter)
	-> fmt::Result
	{
		let Layout(ref lower, _) = *self;
		lower.fmt(f)
	}
}

impl fmt::Display for Layer
{
	fn fmt(&self, f: &mut fmt::Formatter)
	-> fmt::Result
	{
		let Layer(KeyMap(ref layer)) = *self;
		write!(f, "{} {} {} {} {} | {} {} {} {} {} {}
{} {} {} {} {} | {} {} {} {} {} {}
{} {} {} {} {} | {} {} {} {} {}
        {}",
			layer[0], layer[1], layer[2], layer[3], layer[4],
			layer[5], layer[6], layer[7], layer[8], layer[9], layer[10],
			layer[11], layer[12], layer[13], layer[14], layer[15],
			layer[16], layer[17], layer[18], layer[19], layer[20], layer[21],
			layer[22], layer[23], layer[24], layer[25], layer[26],
			layer[27], layer[28], layer[29], layer[30], layer[31],
			layer[32])
	}
}