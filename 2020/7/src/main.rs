use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;

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
    let bags = parser(buffer);
    let count = counter(bags, "shiny gold bag");
    println!("count: {}", count);
    count
}

fn part2(buffer: &String) -> usize {
    println!("Part 1 Beginning.");
    let bags = parser(buffer);
    let contains = contains(bags, "shiny gold bag");
    println!("contains: {}", contains);
    contains
}

#[derive(Debug)]
struct Bag {
    this: String,
    inside: HashMap<String, usize>,
}

// fn parser(buffer: &String, vecs: &mut HashMap<String, Vec<String>>) -> Vec<Bag> {
fn parser(buffer: &String) -> Vec<Bag> {
    let outside = Regex::new(r"^([\w\s]+bag)s contain").unwrap();
    let inside = Regex::new(r"(?m)((\d) ([\w\s]+bag))").unwrap();
    let vstr: Vec<&str> = buffer.split("\n").collect(); // split by bag
    let mut bags: Vec<Bag> = vec!(); // ready a 
    for x in vstr {
        let mut bag: Bag = Bag{
            this: "".to_string(),
            inside: HashMap::new(),
        };
        let outside_bag = outside.captures(x).unwrap().get(1).unwrap().as_str();
        bag.this = outside_bag.to_string();
        for y in inside.find_iter(x) {
            let y = y.as_str();
            // println!("inside_bag: {:?}\n", y);
            let caps = inside.captures(y).unwrap();
            // println!("caps: {:?}\n", caps);
            let insider = caps.get(3).unwrap().as_str().to_string();
            let count = caps.get(2).unwrap().as_str().parse().unwrap();
            bag.inside.insert(insider, count);
        }
        // println!("bag: {:?}", bag);
        bags.push(bag);
    }
    // println!("bags: {:?}", bags);
    bags
}

fn counter(bags: Vec<Bag>, bag: &str) -> usize {
    let potential: HashSet<String> = recursive_counter(&bags, bag);
    potential.len()
}

fn recursive_counter(bags: &Vec<Bag>, current: &str) -> HashSet<String> {
    let mut holder: HashSet<String> = HashSet::new();
    for x in bags {
        for (k,_v) in x.inside.clone() {
            if current == k {
                holder.insert(x.this.as_str().to_string());
                let potential: HashSet<String> = recursive_counter(bags, &x.this.as_str());
                for z in potential {
                    holder.insert(z);
                }
            }
        }
    }
    // println!("holder: {:?}", holder);
    holder
}

fn contains(bags: Vec<Bag>, bag: &str) -> usize {
    let potential: usize = recursive_contains(&bags, bag)-1;
    potential
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