use std::io::{self, Read};

fn main() -> io::Result<()> {
    let buffer = readin();
    //println!("{:?}", buffer);
    let mut vstr: Vec<&str> = buffer.split('\n').collect();
    vstr.pop();
    let vstr: Vec<&str> = vstr;
    //println!("{:?}", vstr);
    let convert = |x: &str| String::from(x).parse().unwrap();
    let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    //println!("{:?}", vint);
    println!("{:?}", part1(&vint));
    println!("{:?}", part2(&vint));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(input: &Vec<u32>) -> u32 {
}

fn part2(input: &Vec<u32>) -> u32 {
}
