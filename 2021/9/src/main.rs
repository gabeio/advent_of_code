#[macro_use]
extern crate log;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let vstr: Vec<&str> = buffer.split('\n').collect();
    trace!("vstr {:?}", &vstr);
    let vstr: Vec<Vec<char>> = vstr.iter().map(|e| {
        e.chars().collect()
    }).collect();
    let convert = |x: char| String::from(x).parse().unwrap();
    let vconvert = |x: Vec<char>| x.clone().into_iter().map(convert).collect();
    let vint: Vec<Vec<u32>> = vstr.clone().into_iter().map(vconvert).collect();
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

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    //
    0
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    0
}
