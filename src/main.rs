mod intcode;
extern crate itertools;
#[macro_use]
mod solutions;

use solutions::day6::*;

fn main() {
    let input: OrbitTree = solutions::get_input(6).unwrap();
    let _tree: OrbitTree = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
        .parse()
        .unwrap();
    //println!("{:?}", tree);
    println!("{:?}", input.path(&"YOU".to_owned(), &"SAN".to_owned()));
}
