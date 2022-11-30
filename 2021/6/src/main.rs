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
    let vint: Vec<u8> = vstr.to_vec().into_iter().map(convert).collect();
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

fn fish(input: &Vec<u8>, day: usize) -> u64 {
    let fish = input.clone();
    debug!("fish {:?}", &fish);
    let mut groups: BTreeMap<u8,u64> = BTreeMap::new();
    for f in &fish {
        if groups.contains_key(&f) {
            if let Some(z) = groups.get_mut(&f) {
                *z += 1;
            } else {
                groups.insert(*f, 1);
            }
        } else {
            groups.insert(*f, 1);
        }
    }
    debug!("groups {:?}", &groups);
    for day in 0..day {
        let mut next: BTreeMap<u8,u64> = BTreeMap::new();
        for (k, v) in &groups {
            if *k == 0u8 {
                if let Some(z) = next.get_mut(&8) {
                    *z += *v; // create count of 0 as 8
                } else {
                    next.insert(8, *v);
                }
                if let Some(z) = next.get_mut(&6) {
                    *z += *v; // reset parents
                } else {
                    next.insert(6, *v);
                }
            } else {
                if let Some(z) = next.get_mut(&(k-1)) {
                    *z += *v; // reset parents
                } else {
                    next.insert(k-1, *v);
                }
            }
        }
        debug!("day {} groups {:?}", &day, &groups);
        debug!("day {} next   {:?}", &day, &next);
        groups = next;
    }
    let mut count = 0;
    for (_, v) in &groups {
        //
        count += *v as u64;
    }
    count
}

fn part1(input: &Vec<u8>) -> u64 {
    fish(input, 80usize)
}

#[allow(unused_variables)]
fn part2(input: &Vec<u8>) -> u64 {
    fish(input, 256usize)
}
