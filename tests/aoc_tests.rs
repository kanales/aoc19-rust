extern crate aoc19;

use aoc19::solutions::run_day;

#[test]
fn day1_1_test() {
    assert_eq!(run_day(1, 1).unwrap(), 3464735);
}

#[test]
fn day1_2_test() {
    assert_eq!(run_day(1, 2).unwrap(), 5194211);
}

#[test]
fn day2_1_test() {
    assert_eq!(run_day(2, 1).unwrap(), 4330636);
}

#[test]
fn day2_2_test() {
    assert_eq!(run_day(2, 2).unwrap(), 6086);
}

#[test]
fn day3_1_test() {
    assert_eq!(run_day(3, 1).unwrap(), 232);
}

#[test]
fn day3_2_test() {
    assert_eq!(run_day(3, 2).unwrap(), 6084);
}

#[test]
fn day4_1_test() {
    assert_eq!(run_day(4, 1).unwrap(), 1716);
}

#[test]
fn day4_2_test() {
    assert_eq!(run_day(4, 2).unwrap(), 1163);
}

#[test]
fn day5_1_test() {
    assert_eq!(run_day(5, 1).unwrap(), 13787043);
}

#[test]
fn day5_2_test() {
    assert_eq!(run_day(5, 2).unwrap(), 3892695);
}

/*#[test]
fn day6_1_test() {
    unimplemented!() //assert_eq!(run_day(2, 1).unwrap(), {});
}

#[test]
fn day6_2_test() {
    unimplemented!() //assert_eq!(run_day(2, 2).unwrap(), {});
}

#[test]
fn day7_1_test() {
    unimplemented!() //assert_eq!(run_day(2, 1).unwrap(), panic!("undefined value"));
}

#[test]
fn day7_2_test() {
    unimplemented!() //assert_eq!(run_day(2, 2).unwrap(), panic!("undefined value"));
}*/
