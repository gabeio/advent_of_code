#[macro_use]
extern crate log;
extern crate aoc;

#[allow(unused_imports)]
use aoc::{
    readin,
    int_grid,
    string_grid,
    u8_grid,
    int_list,
    str_list,
    string_list,
};

#[allow(unused_imports)]
use std::collections::BTreeMap;

#[allow(unused_imports)]
use std::cmp::{min, max};

fn main() {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let result = u8_grid(&buffer, '\n');
    trace!("result {:?}", &result);
    println!("part 1: {:?}", part1(&result));
    println!("part 2: {:?}", part2(&result));
}

const lower: u8 = 96;
const upper: u8 = 38;

fn unique<T: Clone + PartialEq>(a: Vec<T>) -> Vec<T> {
    let mut a = a.clone();
    for x in (0..a.len()).rev() {
      for y in (x+1..a.len()).rev() {
        if &a[x] == &a[y] {
          a.remove(y);
        }
      }
    }
    a
}

fn adjust(a: u8) -> u8 {
    if a > 97 {
        a  - lower
    } else {
        a - upper
    }
}

fn part1(input: &Vec<Vec<u8>>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..input.len() {
        let one = input[i][0..((input[i].len())/2)].to_vec();
        let two = input[i][((input[i].len())/2)..].to_vec();
        if one.len() != two.len() { panic!("not equal"); }
        debug!("one {:?}", one);
        debug!("two {:?}", two);
        let mut one = unique(one);
        let mut two = unique(two);
        debug!("uone {:?}", one);
        debug!("utwo {:?}", two);
        let swap;
        let range = if one.len() > two.len() {
            one.len()
        } else {
            swap = two;
            two = one;
            one = swap;
            one.len()
        };
        for j in 0..range {
            if two.contains(&one[j]) {
                total += adjust(one[j]) as u32;
                break;
            }
        }
    }
    total
}

fn part2(input: &Vec<Vec<u8>>) -> u32 {
    let mut total: u32 = 0;
    for i in (0..input.len()).step_by(3) {
        let one = input[i].to_vec();
        let two = input[i+1].to_vec();
        let three = input[i+2].to_vec();
        debug!("one {:?}", one);
        debug!("two {:?}", two);
        debug!("three {:?}", three);
        let one = unique(one);
        let two = unique(two);
        let three = unique(three);
        debug!("uone {:?}", one);
        debug!("utwo {:?}", two);
        debug!("uthree {:?}", three);
        let mut current = 0;
        for j in 0..one.len() {
            if two.contains(&one[j]) && three.contains(&one[j]) {
                info!("matched: {:?}", String::from_utf8(vec![one[j]]));
                let adjusted = adjust(one[j]) as u32;
                current += adjusted;
                total += adjusted;
                break;
            }
        }
        if current == 0 {
            panic!("no match");
        }
    }
    total
}
