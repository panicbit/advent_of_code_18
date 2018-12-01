#[macro_use] extern crate aoc;

#[aoc(2018, 01, 1)]
fn main(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|freq| freq.parse::<i32>().unwrap())
        .sum::<i32>()
}
