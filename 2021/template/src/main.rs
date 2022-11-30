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
    let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
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

fn convert(x: &str) -> u32 {
    return x.parse().unwrap();
}

fn part1(input: &Vec<u32>) -> u32 {
}

fn part2(input: &Vec<u32>) -> u32 {
}
