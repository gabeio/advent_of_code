use std::io::{self, Read};

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let buffer = readin();
        assert_eq!(part1(&buffer), 2);
    }

    #[test]
    fn part2_test() {
        let buffer = readin();
        assert_eq!(part2(&buffer), 2);
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    println!("part1: {}", part1(&buffer));
    println!("part2: {}", part2(&buffer));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(buffer: &String) -> u32 {
    let Passports: Vec<Passport> = parser(buffer);
    let count: u32 = count(Passports);
    return count;
}

struct Passport {
    byr: Option<i16>, // birth year
    iyr: Option<i16>, // issue year
    eyr: Option<i16>, // expiration year
    hgt: Option<String>, // height
    hcl: Option<String>, // hair color
    ecl: Option<String>, // eye color
    pid: Option<String>, // passport id (optional)
    cid: Option<i64>, // country id
}

impl Default for Passport {
    fn default() -> Passport {
        Passport{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

fn parser(buffer: &String) -> Vec<Passport> {
    let re = Regex::new(r"(?m)([a-z]{3}:[#a-z0-9]+[\s\n]?)+[\n\n]?").unwrap();
    let mut passports: Vec<Passport> = vec!();
    for group in re.captures_iter(buffer) {
        let mut passport: Passport = Default::default();
        let one_passport = group.get(0).unwrap().as_str();
        let one_passport: Vec<&str> = one_passport.split(&[' ', '\n'][..]).collect();
        // 'asdf: 
        for x in one_passport {
            if x != "" {
                let attribute: Vec<&str> = x.split(':').collect();
                // println!("{:?} :> {:?}", attribute[0], attribute[1]);
                match attribute[0] {
                    "byr" => passport.byr = Some(attribute[1].parse::<i16>().unwrap()),
                    "iyr" => passport.iyr = Some(attribute[1].parse::<i16>().unwrap()),
                    "eyr" => passport.eyr = Some(attribute[1].parse::<i16>().unwrap()),
                    "hgt" => passport.hgt = Some(String::from(attribute[1])),
                    "hcl" => passport.hcl = Some(String::from(attribute[1])),
                    "ecl" => passport.ecl = Some(String::from(attribute[1])),
                    "pid" => passport.pid = Some(attribute[1].to_string()),
                    // "pid" => {
                    //     let tmp = match attribute[1].parse::<i64>() {
                    //         Ok(n) => n,
                    //         _ => {
                    //             println!("unable to parse pid");
                    //             continue 'asdf;
                    //         },
                    //     };
                    //     passport.pid = Some(tmp);
                    // },
                    "cid" => passport.cid = Some(attribute[1].parse::<i64>().unwrap()),
                    _ => {
                        // println!("{:?} ::: {:?}", attribute[0], attribute[1])
                    },
                }
            }
        }
        passports.push(passport);
        // println!("{:?}", one_passport);
    }
    return passports;
}

fn count(Passports: Vec<Passport>) -> u32 {
    let mut count: u32 = 0;
    for passport in Passports {
        if passport.byr.unwrap_or(-1) != -1 &&
            passport.iyr.unwrap_or(-1) != -1 &&
            passport.eyr.unwrap_or(-1) != -1 &&
            passport.hgt.unwrap_or("".to_string()) != "" &&
            passport.hcl.unwrap_or("".to_string()) != "" &&
            passport.ecl.unwrap_or("".to_string()) != "" &&
            passport.pid.unwrap_or("".to_string()) != "" {
            count += 1;
        }
    }
    return count;
}

fn part2(buffer: &String) -> u32 {
    let Passports: Vec<Passport> = parser_v2(buffer);
    let count: u32 = count(Passports);
    return count;
}

fn parser_v2(buffer: &String) -> Vec<Passport> {
    let re = Regex::new(r"(?m)([a-z]{3}:[#a-z0-9]+[\s\n]?)+[\n\n]?").unwrap();
    let re2 = Regex::new(r"^([0-9]{2,3})(in|cm)$").unwrap();
    let re3 = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let mut passports: Vec<Passport> = vec!();
    for group in re.captures_iter(buffer) {
        let mut passport: Passport = Default::default();
        let one_passport = group.get(0).unwrap().as_str();
        let one_passport: Vec<&str> = one_passport.split(&[' ', '\n'][..]).collect();
        'asdf: for x in one_passport {
            if x != "" {
                let attribute: Vec<&str> = x.split(':').collect();
                // println!("{:?} :> {:?}", attribute[0], attribute[1]);
                match attribute[0] {
                    "byr" => {
                        let tmp = attribute[1].parse::<i16>().unwrap();
                        if tmp >= 1920 && tmp <= 2002 {
                            passport.byr = Some(tmp);
                        } else {
                            // println!("bad byr");
                            continue 'asdf;
                        }
                    },
                    "iyr" => {
                        let tmp = attribute[1].parse::<i16>().unwrap();
                        if tmp >= 2010 && tmp <= 2020 {
                            passport.iyr = Some(tmp);
                        } else {
                            // println!("bad iyr");
                            continue 'asdf;
                        }
                    },
                    "eyr" => {
                        let tmp = attribute[1].parse::<i16>().unwrap();
                        if tmp >= 2020 && tmp <= 2030 {
                            passport.eyr = Some(tmp);
                        } else {
                            // println!("bad eyr");
                            continue 'asdf;
                        }
                    },
                    "hgt" => {
                        let tmp = attribute[1];
                        let captures = match re2.captures(tmp) {
                            Some(n) => {
                                // println!("n: {:?}", n);
                                n
                            },
                            _ => {
                                // println!("bad hgt 4");
                                continue 'asdf;
                            },
                        };
                        let height: u8 = match captures.get(1) {
                            Some(n) => {
                                // println!("asdf: {:?}", n);
                                match n.as_str().parse::<u8>() {
                                    Ok(n) => n,
                                    _ => 0,
                                }
                            },
                            _ => 0,
                        };
                        let scale = match captures.get(2) {
                            Some(n) => { n.as_str() },
                            _ => "",
                        };
                        if scale == "in".to_string() {
                            if height >= 59 && height <= 76 {
                                //
                            } else {
                                // println!("bad hgt 3");
                                continue 'asdf;
                            }
                        } else if scale == "cm".to_string() {
                            if height >= 150 && height <= 193 {
                                //
                            } else {
                                // println!("bad hgt 2");
                                continue 'asdf;
                            }
                        } else {
                            // println!("bad hgt 1");
                            continue 'asdf;
                        }
                        // println!("{:?}", captures);
                        passport.hgt = Some(String::from(attribute[1]));
                    },
                    "hcl" => {
                        let tmp = attribute[1];
                        let captures = match re3.captures(tmp) {
                            Some(n) => {
                                // println!("n: {:?}", n);
                                n
                            },
                            _ => {
                                // println!("bad hcl");
                                continue 'asdf;
                            },
                        };
                        passport.hcl = Some(tmp.to_string());
                    },
                    "ecl" => {
                        let tmp = attribute[1];
                        if tmp.len() == 3 && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&tmp) {
                            //
                        } else {
                            // println!("bad ecl");
                            continue 'asdf;
                        }
                        passport.ecl = Some(tmp.to_string());
                    },
                    "pid" => {
                        if attribute[1].len() != 9 {
                            // println!("bad pid");
                            continue 'asdf;
                        }
                        let tmp = match attribute[1].parse::<i64>() {
                            Ok(n) => n,
                            _ => {
                                // println!("unable to parse pid");
                                continue 'asdf;
                            },
                        };
                        passport.pid = Some(tmp.to_string());
                    },
                    "cid" => passport.cid = Some(attribute[1].parse::<i64>().unwrap()),
                    _ => {
                        // println!("{:?} ::: {:?}", attribute[0], attribute[1])
                    },
                }
            }
        }
        passports.push(passport);
        // println!("{:?}", one_passport);
    }
    return passports;
}