use std::fs;
use paste::paste;
use advent_of_code_2024::*;
use std::time::Instant;

macro_rules! run_days {
	( $( $x:expr), + ) => {
		$(
			{
				paste!{
					let input = fs::read_to_string(concat!("../inputs/", stringify!($x), ".txt")).unwrap();
					let before = Instant::now();
					println!(concat!($x, "a: {}, time: {:?}"), [<day _ $x>]::a(&input), before.elapsed());
					let beforeb = Instant::now();
					println!(concat!($x, "b: {}, time: {:?}"), [<day _ $x>]::b(&input), beforeb.elapsed());
				}
			}
		)*
	}
}

fn main() {
	run_days!(1, 2);
}