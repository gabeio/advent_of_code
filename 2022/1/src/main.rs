#[macro_use]
extern crate log;
extern crate aoc;

#[allow(unused_imports)]
use aoc::{
    readin,
    AOC,
};

#[allow(unused_imports)]
use std::collections::BTreeMap;

fn main() {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let result = buffer.str_list('\n');
    trace!("result {:?}", &result);
    println!("part 1: {:?}", part1(&result));
    println!("part 2: {:?}", part2(&result));
}

fn convert(x: &str) -> u32 {
    return x.parse().unwrap();
}

fn calories(input: &Vec<&str>) -> Vec<u32> {
    let mut elves: Vec<Vec<u32>> = vec![];
    let mut elf: Vec<u32> = vec![];
    for i in 0..input.len() {
        if input[i].len() > 0 {
            elf.push(convert(input[i]));
        } else {
            elves.push(elf);
            elf = vec![];
        }
        debug!("elf {:?}", elf);
    }
    debug!("elves {:?}", elves);
    let mut calories: Vec<u32> = vec![];
    for elf in elves {
        let mut sum = 0;
        for e in elf {
            sum += e;
        }
        calories.push(sum);
    }
    calories.sort();
    calories
}

fn part1(input: &Vec<&str>) -> u32 {
    calories(input).pop().unwrap()
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut cal = calories(input);
    let top3: Vec<u32> = vec![cal.pop().unwrap(), cal.pop().unwrap(), cal.pop().unwrap()];
    top3.iter().sum()
}
