mod intcode;
use intcode::{Intcode, Process};

#[macro_use]
mod solutions;

fn main() {
    let _c: Intcode = solutions::get_input(5).unwrap();
    let mut p = Process::new(_c);
    let output = p.run_with(|_| 1);
    println!("{:?}", output)
}
