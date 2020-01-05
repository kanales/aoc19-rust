use itertools::Itertools;

use crate::intcode::{Intcode, Process, ProcessStatus};
use std::iter::repeat;

#[derive(Debug, Clone)]
pub struct ProcessChain(Vec<Process>);

use std::iter::FromIterator;

impl<'a> FromIterator<Process> for ProcessChain {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Process>,
    {
        ProcessChain(iter.into_iter().collect())
    }
}

impl IntoIterator for ProcessChain {
    type Item = Process;
    type IntoIter = std::vec::IntoIter<Process>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ProcessChain {
    pub fn init(&mut self, phases: &Vec<i32>) {
        let mut it = phases.iter();
        for p in self.0.iter_mut() {
            p.resume();
            let n = it.next().unwrap();
            p.feed(*n);
        }
    }

    pub fn feed(&mut self, value: i32) -> i32 {
        let mut last = value;
        for p in self.0.iter_mut() {
            match p.resume() {
                ProcessStatus::Awaiting(_) => p.feed(last),
                x => panic!(format!("1: Unexpected status {:?}", x)),
            };
            match p.resume() {
                ProcessStatus::Outputting(x) => last = x,
                x => panic!(format!("2: Unexpected status {:?}", x)),
            }
        }
        last
    }

    pub fn do_loop(&mut self, initial: i32) -> i32 {
        let mut memory: Vec<i32> = vec![initial; self.0.len()];
        while self.0.iter().any(|s| s.status() != ProcessStatus::Exit) {
            //println!("{:?}", memory);
            let mut i = 0;
            for p in self.0.iter_mut() {
                if let ProcessStatus::Awaiting(_) = p.resume() {
                    let idx = (i + memory.len() - 1) % memory.len();
                    let prev = memory[idx];
                    p.feed(prev);
                }
                if let ProcessStatus::Outputting(x) = p.resume() {
                    memory[i] = x;
                }
                if p.status() == ProcessStatus::Exit {}
                i += 1;
            }
        }
        memory[memory.len() - 1]
    }
}

pub fn part1(input: &Intcode) -> i32 {
    let p = Process::new(input.clone());
    let chain: ProcessChain = repeat(p).take(5).collect();
    let perm: Vec<Vec<i32>> = (0..=4).permutations(5).collect();
    perm.iter()
        .map(|p| {
            let mut chain: ProcessChain = chain.clone();
            // initialize
            chain.init(p);
            chain.feed(0)
        })
        .max()
        .unwrap()
}

pub fn part2(input: &Intcode) -> i32 {
    let p = Process::new(input.clone());
    let chain: ProcessChain = repeat(p).take(5).collect();
    let perm: Vec<Vec<i32>> = (5..=9).permutations(5).collect();
    perm.iter()
        .map(|p| {
            let mut chain: ProcessChain = chain.clone();
            // initialize
            chain.init(p);
            chain.do_loop(0)
        })
        .max()
        .unwrap()
}

#[test]
pub fn part1_test() {
    let input = super::get_input(7).unwrap();
    assert_eq!(part1(&input), 34852);
}

#[test]
pub fn part2_test() {
    let input = super::get_input(7).unwrap();
    assert_eq!(part2(&input), 44282086);
}
