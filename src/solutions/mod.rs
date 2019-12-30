pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use std::error::Error;
use std::fmt;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn get_input<R>(day: i32) -> Result<R, <R as FromStr>::Err>
where
    R: FromStr,
{
    let mut file = File::open(format!("inputs/{}.txt", day)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.parse()
}

#[derive(Debug)]
pub struct DayParseError {
    day: i32,
    part: i32,
}

impl fmt::Display for DayParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error executing day!")
    }
}

impl Error for DayParseError {
    fn description(&self) -> &str {
        "Error parsing day inputs"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[allow(dead_code)]
pub fn run_day(day: i32, part: i32) -> Result<i32, Box<dyn Error>> {
    let default = DayParseError {
        day: day,
        part: part,
    };
    let res = match day {
        1 => match part {
            1 => day1::part1(&get_input(1)?),
            2 => day1::part2(&get_input(1)?),
            _ => Err(default)?,
        },
        2 => match part {
            1 => day2::part1(&get_input(2)?),
            2 => day2::part2(&get_input(2)?),
            _ => Err(default)?,
        },
        3 => match part {
            1 => day3::part1(&get_input(3)?),
            2 => day3::part2(&get_input(3)?),
            _ => Err(default)?,
        },
        4 => match part {
            1 => day4::part1(&get_input(4)?),
            2 => day4::part2(&get_input(4)?),
            _ => Err(default)?,
        },
        5 => match part {
            1 => day5::part1(&get_input(5)?),
            2 => day5::part2(&get_input(5)?),
            _ => Err(default)?,
        },
        6 => match part {
            1 => day6::part1(&get_input(6)?),
            2 => day6::part2(&get_input(6)?),
            _ => Err(default)?,
        },
        7 => match part {
            1 => day7::part1(&get_input(7)?),
            2 => day7::part2(&get_input(7)?),
            _ => Err(default)?,
        },
        _ => Err(default)?,
    };
    Ok(res)
}
