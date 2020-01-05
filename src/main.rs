mod intcode;
use intcode::{Intcode, Process, ProcessStatus};
extern crate itertools;
#[macro_use]
mod solutions;
use std::iter::repeat;

use solutions::day7::*;

fn main() {
    let input: Intcode = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".parse().unwrap();
    let p = Process::new(input);
    let mut chain: ProcessChain = repeat(p).take(5).collect();
    chain.init(&vec![9, 7, 8, 5, 6]);
    println!("{}", chain.do_loop(0));
}
