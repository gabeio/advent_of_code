use std::io::{self, Read};
use std::collections::VecDeque;

// use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    // use std::io::prelude::*;
    use std::fs;

    #[test]
    fn input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test").expect("Can't read file.");
        assert_eq!(decrypt(&contents, 5).unwrap().unwrap(), 127);
        Ok(())
    }

    #[test]
    fn input2() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test").expect("Can't read file.");
        assert_eq!(contiguous(&contents, 5).unwrap().unwrap(), 62);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer)?;
    part2(&buffer)?;
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(buffer: &String) -> std::io::Result<usize> {
    println!("Part 1 Beginning.");
    let result = decrypt(buffer, 25).unwrap().unwrap();
    println!("{}", result);
    Ok(result)
}

fn part2(buffer: &String) -> std::io::Result<usize> {
    println!("Part 2 Beginning.");
    let result = contiguous(buffer, 25).unwrap().unwrap();
    println!("{}", result);
    Ok(result)
}

fn decrypt(buffer: &String, capacity: usize) -> std::io::Result<Option<usize>> {
    let input: Vec<usize> = buffer.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();
    // println!("usize capacity: {}", capacity);
    let mut preamble: VecDeque<usize> = VecDeque::with_capacity(capacity);
    // println!("len: {}, capacity: {}", preamble.len(), preamble.capacity());
    for x in input {
        // println!("len: {}, capacity: {}", preamble.len(), preamble.capacity());
        if preamble.len() != preamble.capacity() {
            preamble.push_back(x);
            continue; // for clarity
        } else {
            // println!("preamble: {:?} \nx: {:?}\n", preamble, x);
            let mut matches = false;
            'b: for y in &preamble {
                for z in &preamble {
                    // println!("{} == {}+{} => {}", x, y, z, x == (y+z));
                    if x == (y + z) {
                        matches = true;
                        break 'b;
                    }
                }
            }
            if matches {
                preamble.pop_front();
                preamble.push_back(x);
            } else {
                return Ok(Some(x));
            }
        }
    }
    Ok(None)
}

fn contiguous(buffer: &String, capacity: usize) -> std::io::Result<Option<usize>> {
    let invalid = decrypt(buffer, capacity).unwrap().unwrap();
    let input: Vec<usize> = buffer.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut range: std::ops::Range<usize> = std::ops::Range { start: 0, end: 0 };
    // println!("invalid: {}", invalid);
    'outer: for x in 0..input.len() {
        for y in 0..input.len() {
            // println!("x: {}, y: {}", x, y);
            let mut summation = 0;
            for z in x..y {
                summation += input[z];
            }
            // println!("summation: {}, invalid: {}", summation, invalid);
            if summation == invalid {
                range.start = x;
                range.end = y;
                break 'outer;
            }
        }
    }
    if range != (0..0) {
        // println!("range: {:?}", range);
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for x in range {
            if input[x] > max {
                max = input[x];
            }
            if min > input[x] {
                min = input[x];
            }
        }
        // println!("min: {}, max: {}", min, max);
        return Ok(Some(min+max));
    }
    Ok(None)
}