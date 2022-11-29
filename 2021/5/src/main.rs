#[macro_use]
extern crate log;

use std::io::{self, Read};
use std::collections::BTreeMap;
use std::cmp::{min, max};

type Point = (u32, u32);

fn main() -> io::Result<()> {
    env_logger::init();
    let buffer = readin();
    trace!("{:?}", buffer);
    let mut vstr: Vec<&str> = buffer.split('\n').collect();
    vstr.pop(); // remove last empty line
    let vstr: Vec<&str> = vstr;
    trace!("{:?}", vstr);
    println!("part 1: {:?}", part1(&vstr));
    println!("part 2: {:?}", part2(&vstr));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn convert(x: &str) -> u32 {
    return x.parse().unwrap();
}

fn segment(input: &Vec<&str>) -> BTreeMap<(Point, Point), ()> {
    let segments: BTreeMap<(Point, Point), _> = input.iter().map(|x| {
        let a: Vec<&str> = x.split(' ').collect();
        let b: Vec<Point> = a.iter().step_by(2).map(|y| {
            let c: Vec<&str> = y.split(',').collect();
            (convert(c[0]), convert(c[1]))
        }).collect();
        ((b[0], b[1]), ())
    }).collect();
    segments
}

fn horizontal_vertical(input: &Vec<&str>) -> BTreeMap<Point, u32> {
    let segments = segment(input);
    let mut board: BTreeMap<Point, u32> = BTreeMap::new();
    for (k, _) in &segments {
        let one = k.0;
        let two = k.1;
        if one.0 == two.0 {
            let begin = min(one.1, two.1);
            let end = max(one.1, two.1);
            for y in begin..=end {
                if board.contains_key(&(one.0, y)) {
                    if let Some(z) = board.get_mut(&(one.0, y)) {
                        *z += 1;
                    }
                } else {
                    board.insert((one.0, y) , 1);
                }
            }
        }
        if one.1 == two.1 {
            let begin = min(one.0, two.0);
            let end = max(one.0, two.0);
            for x in begin..=end {
                if board.contains_key(&(x, one.1)) {
                    if let Some(z) = board.get_mut(&(x, one.1)) {
                        *z += 1;
                    }
                } else {
                    board.insert((x, one.1) , 1);
                }
            }
        }
    }
    draw_board(&board);
    return board;
}

fn part1(input: &Vec<&str>) -> u32 {
    return count(horizontal_vertical(input));
}

fn diag(input: &Vec<&str>) -> BTreeMap<Point, u32> {
    let segments = segment(&input);
    let mut board: BTreeMap<Point, u32> = horizontal_vertical(&input);
    for (k, _) in &segments {
        let one = k.0;
        let two = k.1;
        let min_x = min(one.0, two.0);
        let max_x = max(one.0, two.0);
        let min_y = min(one.1, two.1);
        let max_y = max(one.1, two.1);
        let diff_x = max_x - min_x;
        let diff_y = max_y - min_y;
        if diff_x == diff_y {
            debug!("one {:?} two {:?}", one, two);
            // loop through and add points
            let mut current = one;
            loop {
                if board.contains_key(&current) {
                    if let Some(z) = board.get_mut(&current) {
                        *z += 1;
                    }
                } else {
                    board.insert(current, 1);
                }
                if current.0 == two.0 && current.1 == two.1 {
                    break;
                }
                if current.0 > two.0 {
                    current.0 -= 1;
                } else {
                    current.0 += 1;
                }
                if current.1 > two.1 {
                    current.1 -= 1;
                } else {
                    current.1 += 1;
                }
            }
        } else {
            debug!("BAD one {:?} two {:?}", one, two);
        }
    }
    draw_board(&board);
    //debug!("board {:?}", board);
    return board;
}

fn count(input: BTreeMap<Point, u32>) -> u32 {
    let mut count = 0;
    for (_, v) in &input {
        if v >= &2 {
            count += 1;
        }
    }
    return count;
}

fn draw_board(input: &BTreeMap<Point, u32>) {
    for y in 0..=9 {
        for x in 0..=9 {
            if input.contains_key(&(x, y)) {
                debug!("{}", input.get(&(x, y)).unwrap());
            } else {
                debug!(".");
            }
        }
        debug!("\n");
    }
}

fn part2(input: &Vec<&str>) -> u32 {
    return count(diag(input));
}
