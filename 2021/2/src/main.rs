use std::io::{self, Read};

fn main() -> io::Result<()> {
    let buffer = readin();
    //println!("{:?}", buffer);
    let mut vstr: Vec<&str> = buffer.split('\n').collect();
    vstr.pop();
    let vstr: Vec<&str> = vstr;
    //println!("{:?}", vstr);
    //let convert = |x: &str| String::from(x).parse().unwrap();
    //let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    //println!("{:?}", vint);
    println!("{:?}", part1(&vstr));
    println!("{:?}", part2(&vstr));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

struct Sub {
    horizontal: u32,
    depth: u32,
    aim: u32
}

fn part1(input: &Vec<&str>) -> u32 {
    let mut sub: Sub = Sub {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for x in input {
        let command: Vec<&str> = x.split(' ').collect();
        let amount: u32 = String::from(command[1]).parse().unwrap();
        let command = command[0];
        match command {
            "forward" => {
                sub.horizontal += amount;
            },
            "down" => {
                sub.depth += amount;
            },
            "up" => {
                sub.depth -= amount;
            },
            _ => {
                panic!("unknown command: {:?}", command);
            }
        }
    }
    println!("horizontal: {}, depth: {}", sub.horizontal, sub.depth);
    return sub.horizontal*sub.depth;
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut sub: Sub = Sub {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for x in input {
        //println!("x: {}", &x);
        let command: Vec<&str> = x.split(' ').collect();
        let amount: u32 = String::from(command[1]).parse().unwrap();
        let command = command[0];
        match command {
            "forward" => {
                sub.horizontal += amount;
                sub.depth += sub.aim * amount;
            },
            "down" => {
                sub.aim += amount;
            },
            "up" => {
                sub.aim -= amount;
            },
            _ => {
                panic!("unknown command: {:?}", command);
            }
        }
        //println!("h: {}, d: {}, a: {}", sub.horizontal, sub.depth, sub.aim);
    }
    println!("horizontal: {}, depth: {}", sub.horizontal, sub.depth);
    return sub.horizontal*sub.depth;
}
