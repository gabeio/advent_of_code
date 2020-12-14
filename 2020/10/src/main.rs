use std::io::{self, Read};
use std::collections::HashMap;

use petgraph::graphmap::UnGraphMap;

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
        assert_eq!(part2(&contents).unwrap(), 8);
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


fn part2(buffer: &String) -> std::io::Result<usize> {
    println!("Part 2 Beginning.");
    let mut vec_usize: Vec<usize> = buffer.split("\n").map(|x| x.parse().unwrap()).collect();
    let last = jolt_jumps(vec_usize.clone()).unwrap().1;
    vec_usize.push(last); // add max as ending jump
    vec_usize.push(0); // add 0 as starting jump
    vec_usize.sort(); // it is faster if sorted
    let mut undirected_graph_map: UnGraphMap<usize, usize> = UnGraphMap::new();
    // let mut vec_nodes = vec!();
    for a in 0..vec_usize.len() {
        undirected_graph_map.add_node(vec_usize[a]);
        for b in a+1..vec_usize.len() {
            // println!("a: {} b: {}", vec_usize[a], vec_usize[b]);
            if vec_usize[b]-vec_usize[a] <= 3 {
                undirected_graph_map.add_edge(vec_usize[a], vec_usize[b], vec_usize[b]-vec_usize[a]);
            }
        }
    }
    // println!("undirected_graph_map: {:?}", undirected_graph_map);
    let mut counter: usize = 0;
    walker(&undirected_graph_map, &mut counter, last, 0);
    println!("Part 2 Answer: {}.", counter);
    Ok(counter)
}

fn walker(graph: &UnGraphMap<usize, usize>, counter: &mut usize, last: usize, current: usize) {
    if current == last {
        *counter += 1;
    }
    for x in graph.edges(current) {
        if x.0 < x.1 {
            // println!("x: {:?}", x);
            walker(&graph, counter, last, x.1);
        }
    }
}
