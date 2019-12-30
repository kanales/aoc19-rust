mod intcode;
use intcode::{run_until_end, Intcode, Process};

#[macro_use]
mod solutions;

use std::iter::repeat;

fn main() {
    let _c: Intcode = solutions::get_input(5).unwrap();
    let mut p = Process::new(_c);
    let mut it = repeat(1);
    let output = run_until_end(&mut p, &mut it);
    println!("{:?}", output)
}
