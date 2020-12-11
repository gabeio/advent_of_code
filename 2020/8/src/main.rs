use std::io::{self, Read};
use std::collections::BTreeMap;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    // use std::io::prelude::*;
    use std::fs;

    #[test]
    fn input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test").expect("Can't read file.");
        assert_eq!(part1(&contents), 5);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer);
    part2(&buffer);
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(buffer: &String) -> isize {
    println!("Part 1 Beginning.");
    let mut ops = parser(buffer);
    let global = execute(&mut ops, Part::Part1);
    println!("global: {}", global);
    global
}

fn part2(buffer: &String) -> isize {
    println!("Part 2 Beginning.");
    let mut ops = parser(buffer);
    let mut stash: (usize,Op);
    let mut res = -1;
    for (key, value) in ops.clone().iter() {
        match value {
            Op::Acc(_i, _b) => continue,
            Op::Nop(i, b) => {
                stash = (*key, Op::Nop(*i, *b));
                ops.insert(*key, Op::Jmp(*i, false));
            },
            Op::Jmp(i, b) => {
                stash = (*key, Op::Jmp(*i, *b));
                ops.insert(*key, Op::Nop(*i, false));
            },
        }
        res = execute(&mut ops.clone(), Part::Part2);
        if res != -1 {
            println!("res: {}", res);
            break;
        } else {
            ops.insert(*key, stash.1);
        }
    }
    println!("global: {}", res);
    res
}

#[derive(Debug)]
enum Op {
    Acc(isize, bool),
    Nop(isize, bool),
    Jmp(isize, bool),
}

impl Clone for Op {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Op::Acc(i,b) => {
                return Op::Acc(i,b);
            },
            Op::Nop(i,b) => {
                return Op::Nop(i,b);
            },
            Op::Jmp(i,b) => {
                return Op::Jmp(i,b);
            }
        }
    }
}

fn parser(buffer: &String) -> BTreeMap<usize,Op> {
    let re = Regex::new(r"^(nop|acc|jmp)\s([+-][0-9]+)$").unwrap();
    let vstr = buffer.split("\n").collect::<Vec<&str>>();
    let mut operations: BTreeMap<usize,Op> = BTreeMap::new();
    let mut counter = 0;
    for x in vstr {
        let caps = re.captures(x).unwrap();
        // println!("caps: {:?}", caps);
        let op: Op = match caps.get(1).unwrap().as_str() {
            "nop" => {
                let i = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Op::Nop(i, false)
            },
            "acc" => {
                let i = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Op::Acc(i, false)
            },
            "jmp" => {
                let i = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Op::Jmp(i, false)
            },
            _ => panic!("unknown operation"),
        };
        operations.insert(counter, op);
        counter += 1;
    }
    operations
}

enum Part{
    Part1,
    Part2
}

fn execute(operations: &mut BTreeMap<usize, Op>, part: Part) -> isize {
    // println!("operations: {:?}", operations);
    let mut global: isize = 0;
    let mut cur: usize = 0;
    'outer: loop {
        // println!("global: {}", global);
        // println!("cur: {}", cur);
        if cur > operations.len()-1 {
            return global;
        }
        let op = operations.get(&cur).unwrap();
        // println!("op: {:?}", op);
        let origin = cur;
        let new_op = match op {
            Op::Nop(i,b) => {
                if *b {
                    // println!("break outer nop");
                    break 'outer;
                }
                cur += 1;
                Op::Nop(*i,true)
            },
            Op::Acc(n, b) => {
                if *b {
                    // println!("break outer acc");
                    break 'outer;
                }
                global += *n;
                cur += 1;
                Op::Acc(*n, true)
            },
            Op::Jmp(n, b) => {
                if *b {
                    // println!("break outer jmp");
                    break 'outer;
                }
                if n > &mut 0 {
                    cur += n.abs() as usize;
                } else {
                    cur -= n.abs() as usize;
                }
                Op::Jmp(*n, true)
            },
        };
        operations.insert(origin, new_op);
    }
    match part {
        Part::Part1 => return global,
        Part::Part2 => return -1,
    }
}