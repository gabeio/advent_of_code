use std::io::{self, Read};

fn main() -> io::Result<()> {
    let buffer = readin();
    //println!("{:?}", buffer);
    let mut vstr: Vec<&str> = buffer.split('\n').collect();
    vstr.pop();
    //let vstr: Vec<&str> = vstr;
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

fn part1(input: &Vec<&str>) -> u32 {
    let count: u32 = input[0].len() as u32;
    let binary = |x: &str| u32::from_str_radix(x, 2).unwrap();
    let binaries: Vec<u32> = input.to_vec().into_iter().map(binary).collect();
    let (most,least) = common(&binaries, count);
    //println!("most : {:32b} {}", most, most);
    //println!("least: {:32b} {}", least, least);
    let res = most*least;
    //println!("resul: {:32b} {}", res, res);
    return res;
}

fn common(input: &Vec<u32>, count: u32) -> (u32,u32) {
    let mut most: u32 = 0;
    let mut least: u32 = 0;
    let mut mask: u32 = 1;
    mask = mask.rotate_left(count-1);
    //println!("mask: {:32b}", mask);
    for _ in 0..count { // over length
        let mut on = 0;
        let mut off = 0;
        for y in input { // over all
            //println!("yyyy: {:08b}", y);
            //println!("mask: {:08b}", mask);
            let one: u32 = y & mask;
            //println!("oone: {:08b}", one);
            let boolean = one.count_ones();
            //println!("bool: {:08b}", boolean);
            if boolean == 1 {
                on += 1;
            } else if boolean == 0 {
                off += 1;
            } else {
                panic!("um {} should be a binary value", y);
            }
        }
        if on == off {
            most = most | mask;
        } else if on > off {
            most = most | mask;
        } else if off > on {
            least = least | mask;
        }
        mask = mask.rotate_right(1);
    }
    return (most, least);
}

fn part2(input: &Vec<&str>) -> u32 {
    let count: u32 = input[0].len() as u32;
    let binary = |x: &str| u32::from_str_radix(x, 2).unwrap();
    let mb: Vec<u32> = input.clone().to_vec().into_iter().map(binary).collect();
    let lb: Vec<u32> = input.clone().to_vec().into_iter().map(binary).collect();
    let mb = filter(mb, count, true);
    let lb = filter(lb, count, false);
    return mb*lb;
}

fn filter(binaries: Vec<u32>, count: u32, oxy: bool) -> u32 {
    let mut binaries = binaries;
    let mut mask: u32 = 1;
    mask = mask.rotate_left(count-1);
    for _ in 0..count { // over length
        let mut on: Vec<u32> = Vec::new();
        let mut off: Vec<u32> = Vec::new();
        for y in &binaries { // over all
            let val: u32 = y.clone() & mask;
            let boolean = val.count_ones();
            if boolean == 1 {
                on.push(y.clone());
            } else if boolean == 0 {
                off.push(y.clone());
            }
        }
        if on.len() == 0 {
            break;
        }
        if off.len() == 0 {
            break;
        }
        if on.len() == off.len() && oxy == true {
            binaries = on;
        } else if on.len() > off.len() && oxy == true {
            binaries = on;
        } else if off.len() > on.len() && oxy == true {
            binaries = off;
        } else if on.len() == off.len() && oxy == false {
            binaries = off;
        } else if off.len() < on.len() && oxy == false {
            binaries = off;
        } else if on.len() < off.len() && oxy == false {
            binaries = on;
        }
        mask = mask.rotate_right(1);
    }
    //println!("len: {}", binaries.len());
    //println!("bin: {:08b}", binaries[0]);
    return binaries[0];
}
