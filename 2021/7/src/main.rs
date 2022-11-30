#[macro_use]
extern crate log;

use std::io::{self, Read};
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let mut vstr: Vec<&str> = buffer.split(',').collect();
    trace!("vstr {:?}", &vstr);
    let last: Vec<&str> = vstr.pop().unwrap().split('\n').collect();
    trace!("last {:?}", &last);
    vstr.push(last[0]);
    let convert = |x: &str| String::from(x).parse().unwrap();
    let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    trace!("vint {:?}", &vint);
    let mut vint = vint.clone();
    vint.sort();
    println!("part 1: {:?}", part1(&vint));
    println!("part 2: {:?}", part2(&vint));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

pub fn sum_from_zero(n: u32) -> u32 {
    (0..n+1).fold(0, |a, b| a + b)
}

fn horizontal_align(input: &Vec<u32>, align: u32, part2: bool) -> u32 {
    let mut cost: u32 = 0;
    for i in input {
        let now: i32 = *i as i32 - align as i32;
        let now: u32 = now.abs() as u32;
        if part2 {
            cost += sum_from_zero(now);
        } else {
            cost += now;
        }
        debug!("ha: i {} - align {} = cost {} ; sum {}", i, align, now, cost);
    }
    cost
}

fn median(numbers: &Vec<u32>) -> u32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    debug!("numbers: {:?}", numbers);
    let mid = numbers.len() / 2;
    debug!("mid: {}", mid);
    numbers[mid]
}

fn mode(numbers: &Vec<u32>) -> u32 {
    let mut occurrences = BTreeMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn part1(input: &Vec<u32>) -> u32 {
    let min = *input.iter().min().unwrap();
    info!("min: {}", min);
    let max = *input.iter().max().unwrap();
    info!("max: {}", max);
    let mut least = std::u32::MAX;
    for i in min..max {
        debug!("part1: i {}", i);
        let cost = horizontal_align(input, i, false);
        debug!("part1: cost {} least {}", cost, least);
        if cost < least {
            least = cost;
        }
    }
    info!("least: {}", least);
    horizontal_align(input, median(input), false)
}

fn part2(input: &Vec<u32>) -> u32 {
    info!("median least: {}", horizontal_align(input, median(input), true));
    info!("mode least: {}", horizontal_align(input, mode(input), true));
    let min = *input.iter().min().unwrap();
    info!("min: {}", min);
    let max = *input.iter().max().unwrap();
    info!("max: {}", max);
    let mut least = std::u32::MAX;
    for i in min..max {
        debug!("part2: i {}", i);
        let cost = horizontal_align(input, i, true);
        debug!("part1: cost {} least {}", cost, least);
        if cost < least {
            least = cost;
        }
    }
    info!("least: {}", least);
    least
}
