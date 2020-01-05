use crate::intcode::{Intcode, Process, ProcessStatus};

pub fn part1(input: &Intcode) -> i32 {
    let mut p = Process::new(input.clone());
    p.resume(); // start process
    p.feed(1); // give command

    let mut last = 0;
    while let ProcessStatus::Outputting(x) = p.resume() {
        last = x;
    }
    last
}

pub fn part2(input: &Intcode) -> i32 {
    let mut p = Process::new(input.clone());

    p.resume(); // start process
    p.feed(5); // give command
    if let ProcessStatus::Outputting(x) = p.resume() {
        x
    } else {
        panic!("Not outputing")
    }
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
