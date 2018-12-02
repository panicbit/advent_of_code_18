#[macro_use] extern crate aoc;
extern crate itertools;

use itertools::Itertools;

#[aoc(2018, 02, 1)]
fn main(input: &str) -> usize {
    let words = input
        .split_whitespace();

    let num_two_dupes = words
        .clone()
        .filter(|word| has_exact_num_of_dupes(word, 2))
        .count();

    let num_three_dupes = words
        .filter(|word| has_exact_num_of_dupes(word, 3))
        .count();

    num_two_dupes * num_three_dupes
}

fn has_exact_num_of_dupes(word: &str, num: usize) -> bool {
    word.chars()
        .sorted()
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .any(|(_, dupes)| dupes.count() == num)
}