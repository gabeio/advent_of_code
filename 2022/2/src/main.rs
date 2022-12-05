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
    println!("part 1: {:?}", part1(&result));
    println!("part 2: {:?}", part2(&result));
}

/*
 * Rock (A,X) 1
 * Paper (B,Y) 2
 * Scissors (C,Z) 3
 */

fn round(a: char, b: char) -> u32 {
    match (a,b) {
        ('C', 'X') => {
            1 + 6
        },
        ('A', 'Y') => {
            2 + 6
        },
        ('B', 'Z') => {
            3 + 6
        },
        ('A', 'X') => {
            1 + 3
        },
        ('B', 'Y') => {
            2 + 3
        },
        ('C', 'Z') => {
            3 + 3
        }
        (_, 'X') => {
            1
        },
        (_, 'Y') => {
            2
        },
        (_, 'Z') => {
            3
        },
        _ => panic!("unmatched char pair"),
    }
}

fn part1(input: &Vec<&str>) -> u32 {
    let mut total = 0;
    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();
        let score = round(chars[0], chars[chars.len()-1]);
        total += score;
        debug!("round {} {} = {}", chars[0], chars[chars.len()-1], score);
    }
    total
}

/*
 * Rock (A) 1
 * Paper (B) 2
 * Scissors (C) 3
 * lose (X)
 * draw (Y)
 * win  (Z)
 */

fn rounder(a: char, b: char) -> u32 {
    match (a, b) {
        ('A', 'X') => {
            // lose
            // rock
            // we choose scissors
            3
        },
        ('B', 'X') => {
            // lose
            // paper
            // we choose rock
            1
        },
        ('C', 'X') => {
            // lose
            // scissors
            // we choose paper
            2
        },
        ('A', 'Y') => {
            // tie
            // we choose rock
            1 + 3
        },
        ('B', 'Y') => {
            // tie
            // we choose paper
            2 + 3
        },
        ('C', 'Y') => {
            // tie
            // we choose scissors
            3 + 3
        }
        ('A', 'Z') => {
            // win
            // rock
            // we choose paper
            2 + 6
        },
        ('B', 'Z') => {
            // win
            // paper
            // we choose scissors
            3 + 6
        },
        ('C', 'Z') => {
            // win
            // scissors
            // we choose rock
            1 + 6
        },
        _ => panic!("unmatched char pair"),
    }
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut total = 0;
    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();
        let score = rounder(chars[0], chars[chars.len()-1]);
        total += score;
        debug!("round {} {} = {}", chars[0], chars[chars.len()-1], score);
    }
    total
}
