use std::io::{self, Read};
use std::collections::HashMap;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let input: String = "abc

a
b
c

ab
ac

a
a
a
a

b".to_string();
        println!("{:?}", input);
        assert_eq!(summary_any_forms(&input), 11);
    }

    #[test]
    fn input2() {
        let input: String = "abc

a
b
c

ab
ac

a
a
a
a

b".to_string();
        println!("{:?}", input);
        assert_eq!(summary_all_forms(&input), 6);
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

fn part1(buffer: &String) {
    println!("Part 1 Beginning.");
    println!("{}", summary_any_forms(buffer));
}

fn summary_any_forms(buffer: &String) -> usize {
    let re = Regex::new(r"(?m)((([a-z]+)[\n]?)+)[\n\n]?").unwrap();
    let mut vstr: Vec<String> = vec!();
    for group in re.captures_iter(buffer) {
        vstr.push(group[1].to_string());
    }
    let mut count = 0;
    for x in vstr {
        let vstr: Vec<char> = String::from(x).drain(..).collect(); //x.split("\n").map(|i| i.to_string()).collect();
        let mut taken: Vec<char> = Vec::with_capacity(26);
        for y in vstr {
            if !taken.iter().any(|i| &y == i || &y == &'\n') {
                taken.push(y);
                count += 1;
            }
        }
    }
    count
}

fn part2(buffer: &String) {
    println!("Part 2 Beginning.");
    println!("{}", summary_all_forms(buffer));
}

fn summary_all_forms(buffer: &String) -> usize {
    let re = Regex::new(r"(?m)((([a-z]+)[\n]?)+)[\n\n]?").unwrap();
    let mut group: Vec<String> = vec!();
    for cap in re.captures_iter(buffer) {
        group.push(cap[1].to_string());
    }
    let mut count: usize = 0;
    for x in group {
        count += handle_group(x);
    }
    count
}

fn handle_group(x: String) -> usize {

    println!("x: {:?}", x);


    // split group up by people
    let people: Vec<&str> = x.split("\n").collect();

    // collect first person's response
    let mut first: Vec<char> = vec!();
    for y in String::from(people[0]).drain(..).collect::<Vec<char>>() {
        if y != '\n' {
            first.push(y);
        }
    }
    first.sort();
    // println!("first: {:?}", first);

    let mut all: HashMap<char, usize> = HashMap::new();
    for person in people {
        if person == "" {
            // println!("continue 1 \n");
            continue;
        }

        // collect current person's responses
        for z in String::from(person).drain(..).collect::<Vec<char>>() {
            if z != '\n' {
                if let Some(x) = all.get_mut(&z) {
                    *x += 1;
                } else {
                    all.insert(z, 1);
                }
            }
        }
    }

    let mut max: usize = 0;
    for (_, v) in all.iter() {
        if *v > max {
            max = *v;
        }
    }

    let mut local = 0;
    for (_, v) in all.iter() {
        if *v == max {
            local += v;
        }
    }

    let people: Vec<&str> = x.split("\n").collect();

    let mut current: Vec<char> = vec!();
    for person in people {
        // println!("person: {:?}", person);
        // if nothing in string we can skip as it's just an extra new line
        if person == "" {
            // println!("continue 1 \n");
            continue;
        }

        // collect current person's responses
        for z in String::from(person).drain(..).collect::<Vec<char>>() {
            if z != '\n' {
                current.push(z);
            }
        }

        current.sort();

        // compare current to first
        println!("compare :: {:?} == {:?} ? {:?}", first, current, first == current);
        if first == current {
            // if they match add to local
            // println!("local: {}", local);
            local += 1;
        } else {
            // if they don't match then our entire group didn't answer the same
            // println!("continue 3 \n");
            return 0;
        }
        current = vec!();
    }

    println!("local: {}", local);

    local
}