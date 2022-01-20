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
    let mut increases: u32 = 0;
    let mut last: u32 = input[0];
    for x in input {
        if *x == last {
            continue;
        } else if *x > last {
            increases += 1;
        }
        last = *x;
    }
    return increases;
}

fn part2(input: &Vec<u32>) -> u32 {
    let mut window: Vec<u32> = Vec::new();
    let mut last = 0;
    let mut increases = 0; // ignore first "increase"
    for x in input {
        window.insert(0, *x);
        if &window.len() == &3usize {
            let mut sum = 0;
            for y in &window {
                sum += y;
            }
            if sum > last {
                increases += 1;
            }
            //println!("sum: {}, last: {}, increases: {}", sum, last, increases);
            last = sum;
            window.pop();
        }
    }
    return increases-1;
}
