use std::io::{self, Read};
use std::collections::HashMap;
use petgraph::graphmap::UnGraphMap;
use futures::executor::block_on;
use async_recursion::async_recursion;

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
        // let contents: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(block_on(part2(&contents)).unwrap(), 8);
        Ok(())
    }

    #[test]
    fn part2_input2() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input_test2").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(block_on(part2(&contents)).unwrap(), 19208);
        Ok(())
    }

    #[test]
    fn part2_input3() -> std::io::Result<()> {
        let contents = fs::read_to_string("./input").expect("Can't read file.");
        let vus: Vec<usize> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
        assert_eq!(block_on(part2(&contents)).unwrap(), 169255295254528);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer)?;
    block_on(part2(&buffer))?;
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
    println!("Part 1 Answer: {}.", result.0);
    Ok(result.0)
}

fn jolt_jumps(mut vus: Vec<usize>) -> std::io::Result<(usize, usize)> {
    vus.sort();
    // println!("vus: {:?}", vus);
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
    // println!("differences: {:?}", differences);
    Ok((differences[&1]*differences[&3], current as usize+3))
}

fn bump_differences(differences: &mut HashMap<usize,usize>, num: usize) {
    if let Some(x) = differences.get_mut(&num) {
        *x += 1;
    } else {
        differences.insert(num, 1);
    }
}


async fn part2(buffer: &String) -> std::io::Result<usize> {
    println!("Part 2 Beginning.");
    let mut vec_usize: Vec<usize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let last = jolt_jumps(vec_usize.clone()).unwrap().1; // get our "laptop adaptor's" value since we must end here
    vec_usize.push(last); // add max as ending jump
    vec_usize.push(0); // add 0 as starting jump
    vec_usize.sort(); // everything is faster if sorted
    let mut graph_map: UnGraphMap<usize, usize> = UnGraphMap::new(); // empty graph
    for a in 0..vec_usize.len() { // for all points
        graph_map.add_node(vec_usize[a]); // add this point to graph
        for b in a+1..vec_usize.len() { // for all *following* points
            if vec_usize[b]-vec_usize[a] <= 3 { // if they are under a distance of 3
                graph_map.add_edge(vec_usize[a], vec_usize[b], vec_usize[b]-vec_usize[a]); // add a "route" between me and following point
            }
        }
    }
    let mut memoize_cache: HashMap<usize, usize> = HashMap::new();
    let counter = walker(&mut memoize_cache, &graph_map, last, 0).await;
    println!("Part 2 Answer: {}.", counter);
    Ok(counter)
}

#[async_recursion]
async fn walker(memoize_cache: &mut HashMap<usize, usize>, graph_map: &UnGraphMap<usize, usize>, last: usize, current: usize) -> usize {
    match memoize_cache.get(&current).map(|entry| entry.clone()) {
        Some(result) => return result, // if we have already done this don't bother doing it again...
        None => {
            let mut counter: usize = 0; // start our local running total
            if current == last { // we found the end so return a successful walk
                return 1 // return 1 because we only found 1 unique route through the graph
            }
            for x in graph_map.edges(current) { // go through every graph connection
                if x.0 < x.1 { // only visit if we are going forward not backwards
                    let res = walker(memoize_cache, &graph_map, last, x.1).await; // recursively walk tree
                    counter += res; // add how many walks we found to our local running total
                }
            }
            println!("counter: {}", counter);
            memoize_cache.insert(current, counter.clone()); // memoize our input and output for huge performance boost
            return counter // return local running total
        }
    }
}
