#[macro_use] extern crate aoc;
extern crate itertools;

use itertools::Itertools;

#[aoc(2018, 02, 2)]
fn main(input: &str) -> String {
    let words = input
        .split_whitespace();

    let (a, b) = words.clone()
        .cartesian_product(words)
        .find(|(a, b)| words_differ_by_one(a, b))
        .expect("No word pair with only one different letter");

    common_letters(a, b)
}

fn words_differ_by_one(a: &str, b: &str) -> bool {
    let num_differing = a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a != b)
        .count();

    num_differing == 1
}

fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}
