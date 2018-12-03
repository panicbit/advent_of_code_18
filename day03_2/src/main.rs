#[macro_use] extern crate aoc;
extern crate regex;

use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[aoc(2018, 03, 2)]
fn main(input: &str) -> u32 {
    let mut claimed = HashMap::<(u32, u32), usize>::new();
    let mut rectangles = input
        .lines()
        .map(|line| line.parse::<Rectangle>().unwrap());

    for rectangle in rectangles.clone() {
        for claim in rectangle.claims() {
            *claimed.entry(claim).or_insert(0) += 1;
        }
    }

    let unique_claim_id = rectangles
        .find(|rectangle| rectangle
            .claims()
            .all(|claim| claimed.get(&claim) == Some(&1))
        )
        .unwrap()
        .id;

    unique_claim_id
}

#[derive(Copy, Clone)]
struct Rectangle {
    id: u32,
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
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let caps = re.captures(input.trim()).unwrap();

        Ok(Self {
            id: caps[1].parse().unwrap(),
            left: caps[2].parse().unwrap(),
            top: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap(),
        })
    }
}
