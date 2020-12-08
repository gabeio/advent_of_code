use std::io::{self, Read};

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buffer = readin();
        assert_eq!(part1(&buffer), 2);
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer);
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(buffer: &String) -> i32 {
    let Passports: Vec<Passport> = parser(buffer);
    return 0;
}

struct Passport {
    byr: i16, // birth year
    iyr: i16, // issue year
    eyr: i16, // expiration year
    hgt: String, // height
    hcl: String, // hair color
    ecl: String, // eye color
    pid: i16, // passport id (optional)
    cid: i16, // country id
}

fn parser(buffer: &String) -> Vec<Passport> {
    let re = Regex::new(r"(?m)([a-z]{3}:[#a-z0-9]+[\s\n]?)+[\n\n]?").unwrap();
    for group in re.captures_iter(buffer) {
        let one_passport = group.get(0).unwrap().as_str();
        let vstr: Vec<&str> = one_passport.split(" ").collect();
        let mut v_str: Vec<&str> = vec!();
        for x in vstr {
            let v: Vec<&str> = x.split("\n").collect();
            for y in v {
                if y != "" {
                    v_str.push(y);
                }
            }
        }
        println!("{:?}", v_str);
    }
    return vec!();
}