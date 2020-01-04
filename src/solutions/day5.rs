use crate::intcode::{Intcode, Process};

pub fn part1(input: &Intcode) -> i32 {
    Process::new(input.clone()).run_with(|_| 1).unwrap()
}

pub fn part2(input: &Intcode) -> i32 {
    Process::new(input.clone()).run_with(|_| 5).unwrap()
}

#[test]
pub fn part1_test() {
    let input = super::get_input(5).unwrap();
    assert_eq!(part1(&input), 13787043);
}

#[test]
pub fn part2_test() {
    let input = super::get_input(5).unwrap();
    assert_eq!(part2(&input), 3892695);
}
