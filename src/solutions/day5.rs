use crate::intcode::{run_until_end, Intcode, Process};

pub fn part1(input: &Intcode) -> i32 {
    let mut process = Process::new(input.clone());
    let res = run_until_end(&mut process, &mut Some(1).into_iter());
    res.unwrap()
}

pub fn part2(input: &Intcode) -> i32 {
    let mut process = Process::new(input.clone());
    let res = run_until_end(&mut process, &mut Some(5).into_iter());
    res.unwrap()
}
