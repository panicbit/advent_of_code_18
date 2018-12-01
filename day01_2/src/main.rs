#[macro_use] extern crate aoc;

use std::collections::HashSet;

#[aoc(2018, 01, 2)]
fn main(input: &str) -> i32 {
    let mut current_freq = 0;
    let mut freqs = HashSet::new();
    let changes = input
        .split_whitespace()
        .map(|freq| freq.parse::<i32>().unwrap())
        .cycle();

    freqs.insert(0);

    for change in changes {
        current_freq += change;

        if freqs.contains(&current_freq) {
            return current_freq;
        }

        freqs.insert(current_freq);
    }

    unreachable!()
}
