use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    // use std::io::prelude::*;
    use std::fs;

    #[test]
    fn part1_input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test1").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap().0, 35);
        Ok(())
    }

    #[test]
    fn part1_input2() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test2").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap().0, 220);
        Ok(())
    }

    #[test]
    fn part1_input3() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_jumps(vus).unwrap().0, 2201);
        Ok(())
    }

    #[test]
    fn part2_input1() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test1").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_sets(vus).unwrap(), 8);
        Ok(())
    }

    #[test]
    fn part2_input2() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test2").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(jolt_sets(vus).unwrap(), 19208);
        Ok(())
    }

    // #[test]
    // fn part2_input3() -> std::io::Result<()> {
    //     let contents = fs::read_to_string("./input").expect("Can't read file.");
    //     let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
    //     assert_eq!(jolt_sets(vus).unwrap(), 0);
    //     Ok(())
    // }
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
    let vus: Vec<usize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let result = jolt_jumps(vus).unwrap();
    // let result = decrypt(buffer, 25).unwrap().unwrap();
    println!("{:?}", result);
    Ok(result.0)
}

fn part2(buffer: &String) -> std::io::Result<usize> {
    println!("Part 2 Beginning.");
    let vus: Vec<usize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let result = jolt_sets(vus).unwrap();
    // let result = decrypt(buffer, 25).unwrap().unwrap();
    println!("{:?}", result);
    Ok(result)
}

fn jolt_jumps(mut vus: Vec<usize>) -> std::io::Result<(usize, usize)> {
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
    Ok((differences[&1]*differences[&3], current as usize+3))
}

fn jolt_sets(mut vus: Vec<usize>) -> std::io::Result<usize> {
    vus.sort();
    println!("vus: {:?}", vus);
    let mut sets: HashSet<Vec<usize>> = HashSet::new();
    let last = jolt_jumps(vus.clone()).unwrap().1;
    recursive_sets(&mut sets, last, vus.clone());
    Ok(sets.len())
}

fn recursive_sets(sets: &mut HashSet<Vec<usize>>, last: usize, vus: Vec<usize>) {
    println!("vus: {:?}", vus);
    let mut set: Vec<usize> = vec!();
    let mut current = 0;
    for x in 0..(vus.clone().len()) {
        if (vus[x]-current) == 1 {
            set.push(vus[x]);
            let mut vis = vus.clone();
            vis.remove(x);
            recursive_sets(sets, last, vis);
            current = vus[x];
            continue;
        }
        if (vus[x]-current) == 2 {
            set.push(vus[x]);
            let mut vis = vus.clone();
            vis.remove(x);
            recursive_sets(sets, last, vis);
            current = vus[x];
            continue;
        }
        if (vus[x]-current) == 3 {
            set.push(vus[x]);
            let mut vis = vus.clone();
            vis.remove(x);
            recursive_sets(sets, last, vis);
            current = vus[x];
            continue;
        }
        break;
    }
    if current == last {
        println!("good set: {:?}", set);
        sets.insert(set);
    } else {
        println!("failed set: {:?}", set);
    }
}

fn bump_differences(differences: &mut HashMap<usize,usize>, num: usize) {
    if let Some(x) = differences.get_mut(&num) {
        *x += 1;
    } else {
        differences.insert(num, 1);
    }
}
