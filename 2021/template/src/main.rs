#[macro_use]
extern crate log;
extern crate aoc;

#[allow(unused_imports)]
use aoc::{
    readin,
    int_grid,
    string_grid,
    int_list,
    str_list,
    string_list,
};

#[allow(unused_imports)]
use std::collections::BTreeMap;

fn main() {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let result = int_grid(&buffer, '\n');
    trace!("result {:?}", &result);
    println!("part 1: {:?}", part1(&result));
    println!("part 2: {:?}", part2(&result));
}
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    0
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    0
}
