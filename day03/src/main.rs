#[macro_use] extern crate aoc;
extern crate regex;

use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[aoc(2018, 03, 1)]
fn main(input: &str) -> usize {
    let mut claimed = HashMap::<(u32, u32), usize>::new();
    let rectangles = input
        .lines()
        .map(|line| line.parse::<Rectangle>().unwrap());

    for rectangle in rectangles {
        for claim in rectangle.claims() {
            *claimed.entry(claim).or_insert(0) += 1;
        }
    }

    let num_double_claims = claimed.values()
        .filter(|&&num_claimed| num_claimed >= 2)
        .count();

    num_double_claims
}

#[derive(Copy, Clone)]
struct Rectangle {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn claims<'a>(&'a self) -> impl Iterator<Item = (u32, u32)> + 'a {
        (0..self.width).flat_map(move |x|
            (0..self.height).map(move |y|
                (self.left + x, self.top + y)
            )
        )
    }
}

impl FromStr for Rectangle {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let caps = re.captures(input.trim()).unwrap();

        Ok(Self {
            left: caps[1].parse().unwrap(),
            top: caps[2].parse().unwrap(),
            width: caps[3].parse().unwrap(),
            height: caps[4].parse().unwrap(),
        })
    }
}
