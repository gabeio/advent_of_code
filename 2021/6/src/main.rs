#[macro_use]
extern crate log;

use std::io::{self, Read};

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
    let vint: Vec<usize> = vstr.to_vec().into_iter().map(convert).collect();
    trace!("vint {:?}", &vint);
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

#[allow(dead_code)]
fn convert(x: &str) -> u8 {
    return x.parse().unwrap();
}

fn fish(input: &Vec<usize>, day: usize) -> u64 {
    let fish = input.clone();
    debug!("fish {:?}", &fish);
    let mut groups: Vec<u64> = vec![0;9];
    for i in 0..fish.len() {
        groups[fish[i]] += 1;
    }
    debug!("groups {:?}", &groups);
    for day in 0..day {
        let mut next: Vec<u64> = vec![0; 9];
        for i in 0..groups.len() {
            if i == 0 {
                next[8] += groups[i];
                next[6] += groups[i];
            } else {
                next[i-1] += groups[i];
            }
        }
        debug!("day {} groups {:?}", &day, &groups);
        debug!("day {} next   {:?}", &day, &next);
        groups = next;
    }
    let mut count = 0;
    for i in 0..groups.len() {
        count += groups[i];
    }
    count
}

fn part1(input: &Vec<usize>) -> u64 {
    fish(input, 80usize)
}

#[allow(unused_variables)]
fn part2(input: &Vec<usize>) -> u64 {
    fish(input, 256usize)
}
