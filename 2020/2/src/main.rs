use std::io::{self, Read};

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

fn part1(buffer: &String) {
    let vstr: Vec<&str> = buffer.split('\n').collect();
    // println!("{:?}", vstr);
    let mut count = 0;
    for x in vstr {
        if test_password1(x) {
            count += 1;
        }
    }
    println!("part 1 count: {:?}", count);
}

fn test_password1(test: &str) -> bool {
    let convert = |x: &str| String::from(x).parse().unwrap();
    // println!("{:?}", test);
    let vstr: Vec<&str> = test.split(": ").collect();
    // println!("{:?}", vstr);
    let policy: &str = vstr[0];
    // println!("{:?}", policy);
    let password: &str = vstr[1];
    // println!("{:?}", password);
    let vstr: Vec<&str> = policy.split(" ").collect();
    // println!("{:?}", vstr);
    let letter = vstr[1];
    // println!("{:?}", letter);
    let vstr: Vec<&str> = vstr[0].split("-").collect();
    let min_max: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    // println!("{:?}", min_max);
    let min = min_max[0];
    let max = min_max[1];
    let mut count = 0;
    for x in password.chars() {
        if x == letter.chars().next().unwrap() {
            count += 1;
        }
    }
    if count >= min && count <= max {
        // println!("{:?} {:?} {:?} true", policy, password, count);
        return true;
    } else {
        // println!("{:?} {:?} {:?} false", policy, password, count);
        return false;
    }
}

fn part2(buffer: &String) {
    let vstr: Vec<&str> = buffer.split('\n').collect();
    // println!("{:?}", vstr);
    let mut count = 0;
    for x in vstr {
        if test_password2(x) {
            count += 1;
        }
    }
    println!("part 2 count: {:?}", count);
}

fn test_password2(test: &str) -> bool {
    let convert = |x: &str| String::from(x).parse().unwrap();
    // println!("{:?}", test);
    let vstr: Vec<&str> = test.split(": ").collect();
    // println!("{:?}", vstr);
    let policy: &str = vstr[0];
    // println!("{:?}", policy);
    let password: &str = vstr[1];
    // println!("{:?}", password);
    let vstr: Vec<&str> = policy.split(" ").collect();
    // println!("{:?}", vstr);
    let letter = vstr[1].chars().next().unwrap();
    // println!("{:?}", letter);
    let vstr: Vec<&str> = vstr[0].split("-").collect();
    let a_b: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    let a = a_b[0]-1;
    let b = a_b[1]-1;
    // println!("{:?}", a_b);
    if a > (password.len()-1) as u32 || b > (password.len()-1) as u32 {
        return false;
    }
    let a_test: bool = letter == password.chars().nth(a as usize).unwrap();
    let b_test: bool = letter == password.chars().nth(b as usize).unwrap();
    if a_test ^ b_test {
        return true;
    } else {
        return false;
    }
}