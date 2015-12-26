/// Stochaistic optimisation based on simulated annealing.
/// Math is shamelessly taken from: http://mkweb.bcgsc.ca/carpalx/?simulated_annealing
/// This code is written to be generic and can be reused for other applications.

extern crate rand;

use std::f64;
use std::ops::Range;
use self::rand::thread_rng;
use self::rand::Rng;

// These values are taken from Carpalx, with T0 adjusted for the scale that our
// penalty model outputs.
const T0: f64 = 1.5;
const K: f64 = 10.0;
const P0: f64 = 1.0;
const N: usize = 10000;
const KN: f64 = K / (N as f64);

// T(i) = T0 exp(-ik/N)
fn temperature(i: usize) -> f64 {
	T0 * f64::exp(-(i as f64) * KN)
}

// p(dE, i) = p0 exp(-dE/T(i))
fn cutoff_p(de: f64, i: usize) -> f64 {
	let t = temperature(i);
	P0 * f64::exp(-de / t)
}

// For positive dE, accept if r < p_dE where r ~ Uniform(0, 1)
pub fn accept_transition(de: f64, i: usize) -> bool {
	if de < 0.0 {
		true
	} else {
		let p_de = cutoff_p(de, i);
		let r = thread_rng().next_f64();
		r < p_de
	}
}

pub fn get_simulation_range() -> Range<usize> {
	1..(N+1)
}