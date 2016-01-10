#![feature(linked_list_extras)]

mod layout;
mod penalty;
mod annealing;
mod simulator;

extern crate getopts;

use std::env;
use std::fs::File;
use std::io::Read;
use getopts::Options;

fn main()
{
	let mut opts = Options::new();
	opts.optflag("h", "help", "print this help menu");
	opts.optflag("d", "debug", "show debug logging");
	opts.optopt("t", "top", "number of top layouts to print (default: 1)", "TOP_LAYOUTS");
	opts.optopt("s", "swaps-per-iteration", "maximum number of swaps per iteration (default: 3)", "SWAPS");

	let args: Vec<String> = env::args().collect();
	let progname = &args[0];
	if args.len() < 2 {
		print_usage(progname, opts);
		return;
	}
	let command = &args[1];
	let matches = match opts.parse(&args[2..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};

	// --help
	if matches.opt_present("h") {
		print_usage(progname, opts);
		return;
	}

	// Read corpus.
	let corpus_filename = match matches.free.get(0) {
		Some(f) => f,
		None => {
			print_usage(progname, opts);
			return;
		},
	};
	let mut f = match File::open(corpus_filename) {
		Ok(f) => f,
		Err(e) => {
			println!("Error: {}", e);
			panic!("could not read corpus");
		},
	};
	let mut corpus = String::new();
	match f.read_to_string(&mut corpus) {
		Ok(_) => (),
		Err(e) => {
			println!("Error: {}", e);
			panic!("could not read corpus");
		}
	};

	// Read layout, if applicable.
	let _layout;
	let layout = match matches.free.get(1) {
		None => &layout::INIT_LAYOUT,
		Some(layout_filename) => {
			let mut f = match File::open(layout_filename) {
				Ok(f) => f,
				Err(e) => {
					println!("Error: {}", e);
					panic!("could not read layout");
				}
			};
			let mut layout_str = String::new();
			match f.read_to_string(&mut layout_str) {
				Ok(_) => (),
				Err(e) => {
					println!("Error: {}", e);
					panic!("could not read layout");
				}
			};
			_layout = layout::Layout::from_string(&layout_str[..]);
			&_layout
		},
	};

	// Parse options.
	let debug = matches.opt_present("d");
	let top   = numopt(matches.opt_str("t"), 1usize);
	let swaps = numopt(matches.opt_str("s"), 3usize);

	match command.as_ref() {
		"run" => run(&corpus[..], layout, debug, top, swaps),
		"run-ref" => run_ref(&corpus[..]),
		"refine" => refine(&corpus[..], layout, debug, top, swaps),
		_ => print_usage(progname, opts),
	};
}

fn run(s: &str, layout: &layout::Layout, debug: bool, top: usize, swaps: usize)
{
	let penalties = penalty::init();
	let init_pos_map = layout::INIT_LAYOUT.get_position_map();
	let quartads = penalty::prepare_quartad_list(s, &init_pos_map);
	let len = s.len();

	loop {
		simulator::simulate(&quartads, len, layout, &penalties, debug, top, swaps);
	}
}

fn run_ref(s: &str)
{
	let penalties = penalty::init();
	let init_pos_map = layout::INIT_LAYOUT.get_position_map();
	let quartads = penalty::prepare_quartad_list(s, &init_pos_map);
	let len = s.len();

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::QWERTY_LAYOUT, &penalties, true);
	println!("Reference: QWERTY");
	simulator::print_result(&layout::QWERTY_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::DVORAK_LAYOUT, &penalties, true);
	println!("Reference: DVORAK");
	simulator::print_result(&layout::DVORAK_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::COLEMAK_LAYOUT, &penalties, true);
	println!("Reference: COLEMAK");
	simulator::print_result(&layout::COLEMAK_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::QGMLWY_LAYOUT, &penalties, true);
	println!("Reference: QGMLWY");
	simulator::print_result(&layout::QGMLWY_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::WORKMAN_LAYOUT, &penalties, true);
	println!("Reference: WORKMAN");
	simulator::print_result(&layout::WORKMAN_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::MALTRON_LAYOUT, &penalties, true);
	println!("Reference: MALTRON");
	simulator::print_result(&layout::MALTRON_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::MTGAP_LAYOUT, &penalties, true);
	println!("Reference: MTGAP");
	simulator::print_result(&layout::MTGAP_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::CAPEWELL_LAYOUT, &penalties, true);
	println!("Reference: CAPEWELL");
	simulator::print_result(&layout::CAPEWELL_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::ARENSITO_LAYOUT, &penalties, true);
	println!("Reference: ARENSITO");
	simulator::print_result(&layout::ARENSITO_LAYOUT, &penalty);
	println!("");

	let penalty = penalty::calculate_penalty(&quartads, len, &layout::INIT_LAYOUT, &penalties, true);
	println!("Reference: INITIAL");
	simulator::print_result(&layout::INIT_LAYOUT, &penalty);
}

fn refine(s: &str, layout: &layout::Layout, debug: bool, top: usize, swaps: usize)
{
	let penalties = penalty::init();
	let init_pos_map = layout::INIT_LAYOUT.get_position_map();
	let quartads = penalty::prepare_quartad_list(s, &init_pos_map);
	let len = s.len();

	simulator::refine(&quartads, len, layout, &penalties, debug, top, swaps);
}

fn print_usage(progname: &String, opts: Options)
{
	let brief = format!("Usage: {} (run|run-ref) <corpus> [OPTIONS]", progname);
	print!("{}", opts.usage(&brief));
}

fn numopt<T>(s: Option<String>, default: T)
-> T
where T: std::str::FromStr + std::fmt::Display
{
	match s {
		None => default,
		Some(num) => match num.parse::<T>() {
			Ok(n) => n,
			Err(_) => {
				println!("Error: invalid option value {}. Using default value {}.", num, default);
				default
			},
		},
	}
}