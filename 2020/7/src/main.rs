use std::io::{self, Read};
use std::collections::{BTreeMap, BTreeSet};
use std::time::SystemTime;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    // use std::io::prelude::*;
    use std::fs;

    #[test]
    fn input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test").expect("Can't read file.");
        assert_eq!(part1(&contents), 4);
        Ok(())
    }

    #[test]
    fn input2a() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test").expect("Can't read file.");
        assert_eq!(part2(&contents), 32);
        Ok(())
    }

    #[test]
    fn input2b() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test2").expect("Can't read file.");
        assert_eq!(part2(&contents), 126);
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

fn part1(buffer: &String) -> usize {
    println!("Part 1 Beginning.");
    let now = SystemTime::now();
    let bags = parser(buffer);
    let parser = now.elapsed().unwrap();
    let now = SystemTime::now();
    let count = counter(bags, "shiny gold bag");
    let counter = now.elapsed().unwrap();
    println!("count: {}", count);
    println!("now: {:?}, parser: {:?}, counter: {:?}", now, parser, counter);
    count
}

fn part2(buffer: &String) -> usize {
    println!("Part 2 Beginning.");
    let now = SystemTime::now();
    let bags = parser(buffer);
    let parser = now.elapsed().unwrap();
    let now = SystemTime::now();
    let contains = contains(bags, "shiny gold bag");
    let container = now.elapsed().unwrap();
    println!("contains: {}", contains);
    println!("now: {:?}, parser: {:?}, container: {:?}", now, parser, container);
    contains
}

#[derive(Debug)]
struct Bag {
    this: String,
    inside: BTreeMap<String, usize>,
}

fn parser(buffer: &String) -> Vec<Bag> {
    let outside = Regex::new(r"^([\w\s]+bag)s contain").unwrap();
    let inside = Regex::new(r"(?m)((\d) ([\w\s]+bag))").unwrap();
    let vstr: Vec<&str> = buffer.split("\n").collect(); // split by bag
    let mut bags: Vec<Bag> = vec!();
    for x in vstr {
        let mut bag: Bag = Bag{
            this: "".to_string(),
            inside: BTreeMap::new(),
        };
        bag.this = outside.captures(x).unwrap().get(1).unwrap().as_str().to_string();
        for y in inside.find_iter(x) {
            let caps = inside.captures(y.as_str()).unwrap();
            let insider = caps.get(3).unwrap().as_str().to_string();
            let count = caps.get(2).unwrap().as_str().parse().unwrap();
            bag.inside.insert(insider, count);
        }
        bags.push(bag);
    }
    bags
}

fn counter(bags: Vec<Bag>, bag: &str) -> usize {
    let potential: BTreeSet<String> = recursive_counter(&bags, bag);
    potential.len()
}

fn recursive_counter(bags: &Vec<Bag>, current: &str) -> BTreeSet<String> {
    let mut holder: BTreeSet<String> = BTreeSet::new();
    for x in bags {
        for (k,_v) in x.inside.clone() {
            if current == k {
                holder.insert(x.this.as_str().to_string());
                for z in recursive_counter(bags, &x.this.as_str()) {
                    holder.insert(z);
                }
            }
        }
    }
    holder
}

fn contains(bags: Vec<Bag>, bag: &str) -> usize {
    recursive_contains(&bags, bag)-1
}

fn recursive_contains(bags: &Vec<Bag>, current: &str) -> usize {
    let mut count: usize = 1;
    for x in bags {
        if x.this == current {
            for (k, v) in x.inside.clone() {
                count += v * recursive_contains(bags, k.as_str());
            }
        }
    }
    count
}