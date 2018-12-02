#[macro_use] extern crate aoc;

use std::collections::HashSet;

#[aoc(2018, 01, 2)]
fn main(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|freq| freq.parse::<i32>().unwrap())
        .cycle()
        .scan((HashSet::new(), 0), |(freqs, current_freq), freq| {
            *current_freq += freq;

            match freqs.insert(*current_freq) {
                false => Some(Some(*current_freq)),
                true => Some(None),
            }
        })
        .flatten()
        .next()
        .expect("No duplicate frequency found")
}
