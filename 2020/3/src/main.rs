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
    let mut vvstr: Vec<Vec<char>> = vec!();
    for x in vstr {
        vvstr.push(String::from(x).drain(..).collect());
    }
    println!("trees hit: {:?}", sledding(&vvstr, 3, 1));
}

struct Point {
    x: i16,
    y: i16,
}

fn sledding(trees: &Vec<Vec<char>>, right: i16, down: i16) -> u32 {
    let mut current = Point{x: 0,y: 0};
    let mut hits = 0;
    loop {
        // add right and down to current
        let new = Point{x: current.x + right, y: current.y + down};
        let mut adjusted_x = new.x;
        // check if we are out of bounds (left) and we are done
        if new.y > trees.len() as i16 - 1 {
            return hits;
        }
        // check if we are out of bounds (right) and adjust
        if new.x > trees[0].len() as i16 - 1 {
            adjusted_x = adjusted_x % trees[0].len() as i16;
        }
        // see if we have hit a tree
        if trees[new.y as usize][adjusted_x as usize] == '#' {
            hits += 1;
        }
        // update current
        current = new;
    }
}

fn part2(buffer: &String) {
    let vstr: Vec<&str> = buffer.split('\n').collect();
    let mut vvstr: Vec<Vec<char>> = vec!();
    for x in vstr {
        vvstr.push(String::from(x).drain(..).collect());
    }
    let inputs: [[i16; 2];5] = [
        [1,1],
        [3,1],
        [5,1],
        [7,1],
        [1,2]
    ];
    let mut outputs: Vec<u64> = vec!();
    for x in inputs.iter() {
        outputs.push(sledding(&vvstr, x[0], x[1]) as u64);
    }
    println!("outputs: {:?}", outputs);
    let mut result: u64 = outputs[0];
    for x in 1..=4 {
        println!("result: {}", result);
        result *= outputs[x];
    }
    println!("result: {}", result);
}