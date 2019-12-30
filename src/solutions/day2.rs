use crate::intcode::{Intcode, Process};

pub fn part1(input: &Intcode) -> i32 {
    let mut p = Process::new(input.replace(12, 2));
    p.run();
    p.get(0)
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
            p.run();
            p.get(0) == 19690720
        })
        .nth(0)
        .unwrap();
    100 * n + v
}
