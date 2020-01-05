mod intcode;
extern crate itertools;
#[macro_use]
mod solutions;

use solutions::day8::*;

fn main() {
    let input: String = solutions::get_input(8).unwrap();
    //let input = "0222112222120000";
    let v: Option<Vec<u8>> = input.chars().map(digit_to_u8).collect();
    let img = Image::new(6, 25, v.unwrap());

    let flatimg = img.flatten();
    println!("{}", flatimg);
}
