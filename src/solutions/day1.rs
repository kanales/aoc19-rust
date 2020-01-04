#[derive(Debug, PartialEq, Eq)]
pub struct Input(Vec<i32>);
use std::iter::successors;
use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Input, Self::Err> {
        input
            .split("\n")
            .map(|x| x.parse())
            .collect::<Result<Vec<i32>, Self::Err>>()
            .map(|x| Input(x))
    }
}

use std::convert::Into;
impl Into<Vec<i32>> for Input {
    fn into(self) -> Vec<i32> {
        match self {
            Input(c) => c,
        }
    }
}

fn get_fuel(x: &i32) -> i32 {
    x / 3 - 2
}

pub fn part1(input: &Input) -> i32 {
    let vec = &input.0;
    vec.iter().map(get_fuel).sum()
}

fn iter_fuel(weight: &i32) -> i32 {
    successors(Some(*weight), |x| Some(x / 3 - 2))
        .take_while(|x| x > &0)
        .skip(1)
        .sum()
}

pub fn part2(input: &Input) -> i32 {
    let vec = &input.0;
    vec.iter().map(iter_fuel).sum()
}

#[test]
pub fn part1_test() {
    let input = super::get_input(1).unwrap();
    assert_eq!(part1(&input), 3464735);
}

#[test]
pub fn part2_test() {
    let input = super::get_input(1).unwrap();
    assert_eq!(part2(&input), 5194211);
}
