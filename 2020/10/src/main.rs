use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    // use std::io::prelude::*;
    use std::fs;

    #[test]
    fn input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test1").expect("Can't read file.");
        let vus: Vec<isize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap(), 35);
        Ok(())
    }

    #[test]
    fn input2() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test2").expect("Can't read file.");
        let vus: Vec<isize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap(), 220);
        Ok(())
    }

    #[test]
    fn input3() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input").expect("Can't read file.");
        let vus: Vec<isize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap(), 2201);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer)?;
    part2(&buffer)?;
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn part1(buffer: &String) -> std::io::Result<usize> {
    println!("Part 1 Beginning.");
    let vus: Vec<isize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let result = jolt_jumps(vus).unwrap();
    // let result = decrypt(buffer, 25).unwrap().unwrap();
    println!("{:?}", result);
    Ok(result)
}

fn part2(buffer: &String) -> std::io::Result<usize> {
    println!("Part 1 Beginning.");
    let vus: Vec<isize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let result = jolt_sets(vus).unwrap();
    // let result = decrypt(buffer, 25).unwrap().unwrap();
    println!("{:?}", result);
    Ok(result)
}

fn jolt_jumps(mut vus: Vec<isize>) -> std::io::Result<usize> {
    vus.sort();
    println!("vus: {:?}", vus);
    let mut current = 0;
    let mut differences: HashMap<usize, usize> = HashMap::new();
    for x in vus {
        if (x-current) == 1 {
            bump_differences(&mut differences, 1);
            current = x;
            continue;
        }
        if (x-current) == 2 {
            bump_differences(&mut differences, 2);
            current = x;
            continue;
        }
        if (x-current) == 3 {
            bump_differences(&mut differences, 3);
            current = x;
            continue;
        }
        break;
    }
    bump_differences(&mut differences, 3);
    println!("differences: {:?}", differences);
    Ok(differences[&1]*differences[&3])
}

fn jolt_sets(mut vus: Vec<isize>) -> std::io::Result<usize> {
    vus.sort();
    println!("vus: {:?}", vus);
    let mut sets: HashSet<Vec<usize>> = HashSet::new();
    recursive_sets(&mut sets, vus.clone());
    // loop {
    //     let mut set: Vec<usize> = vec!();
    //     loop {
    //         let mut matches = false;
    //         let a_match = ofor(1, &vus, current);
    //         if a_matcher(a_match, &mut differences, &mut current, &mut matches, 1) {
    //             continue;
    //         }
    //         let a_match = ofor(2, &vus, current);
    //         if a_matcher(a_match, &mut differences, &mut current, &mut matches, 2) {
    //             continue;
    //         }
    //         let a_match = ofor(3, &vus, current);
    //         if a_matcher(a_match, &mut differences, &mut current, &mut matches, 3) {
    //             continue
    //         }
    //         if !matches {
    //             break;
    //         }
    //     }
    // }
    Ok(0)
}

fn recursive_sets(sets: &mut HashSet<Vec<usize>>, vus: Vec<isize>) {

}

fn bump_differences(differences: &mut HashMap<usize,usize>, num: usize) {
    if let Some(x) = differences.get_mut(&num) {
        *x += 1;
    } else {
        differences.insert(num, 1);
    }
}
