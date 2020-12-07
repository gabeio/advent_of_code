use std::io::{self, Read};

fn main() -> io::Result<()> {
    let buffer = readin();
    // println!("{:?}", buffer);
    let vstr: Vec<&str> = buffer.split('\n').collect();
    // println!("{:?}", vstr);
    let convert = |x: &str| String::from(x).parse().unwrap();
    let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    // println!("{:?}", vint);
    part1(&vint);
    part2(&vint);
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(input: &Vec<u32>) {
    for x in input {
        for y in input {
            if x + y == 2020 {
                // println!("x{} y{}", x, y);
                println!("{:?}", x*y);
                return
            }
        }
    }
}

fn part2(input: &Vec<u32>) {
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == 2020 {
                    // println!("x{} y{} z{}", x, y, z);
                    println!("{:?}", x*y*z);
                    return
                }
            }
        }
    }
}