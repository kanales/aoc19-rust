use crate::intcode::{Intcode, Process, ProcessStatus};

pub fn part1(input: &Intcode) -> i32 {
    let mut p = Process::new(input.replace(12, 2));
    match p.resume() {
        ProcessStatus::Exit => p.head(),
        _ => panic!("Process is still running"),
    }
}
fn pairs<'a>(input: &'a Vec<i32>) -> Vec<(&'a i32, &'a i32)> {
    input
        .iter()
        .flat_map(|x| input.iter().map(move |y| (x, y)))
        .collect()
}

pub fn part2(input: &Intcode) -> i32 {
    let possibilities: Vec<i32> = (1..=99).collect();
    let ps = pairs(&possibilities);
    let (&n, &v) = ps
        .iter()
        .filter(|(&n, &v)| {
            let mut p = Process::new(input.replace(n, v));
            let res = match p.resume() {
                ProcessStatus::Exit => p.head(),
                _ => panic!("Process is still running"),
            };
            res == 19690720
        })
        .nth(0)
        .unwrap();
    100 * n + v
}

#[test]
pub fn part1_test() {
    let input = super::get_input(2).unwrap();
    assert_eq!(part1(&input), 4330636);
}

#[test]
pub fn part2_test() {
    let input = super::get_input(2).unwrap();
    assert_eq!(part2(&input), 6086);
}
