#[macro_use]
extern crate log;

use std::io::{self, Read};
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    env_logger::init();
    let buffer = readin();
    let buffer = buffer.trim();
    trace!("buffer {:?}", &buffer);
    let vstr: Vec<&str> = buffer.split('\n').collect();
    //vstr.pop();
    trace!("vstr {:?}", &vstr);
    let vstr: Vec<Vec<char>> = vstr.iter().map(|e| {
        e.chars().collect()
    }).collect();
    let convert = |x: char| String::from(x).parse().unwrap();
    let vconvert = |x: Vec<char>| x.clone().into_iter().map(convert).collect();
    let vint: Vec<Vec<u32>> = vstr.clone().into_iter().map(vconvert).collect();
    trace!("vint {:?}", &vint);
    println!("part 1: {:?}", part1(vint.clone()));
    println!("part 2: {:?}", part2(vint.clone()));
    Ok(())
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn check_surroundings(i: usize, j: usize, input: Vec<Vec<u32>>) -> bool {
    if i > 0 {
        if let Some(a) = input.get(i-1) {
            if let Some(b) = a.get(j) {
                if *b <= input[i][j] {
                    return false;
                }
            }
        }
    }
    if let Some(a) = input.get(i+1) {
        if let Some(b) = a.get(j) {
            if *b <= input[i][j] {
                return false;
            }
        }
    }
    if j > 0 {
        if let Some(a) = input.get(i) {
            if let Some(b) = a.get(j-1) {
                if *b <= input[i][j] {
                    return false;
                }
            }
        }
    }
    if let Some(a) = input.get(i) {
        if let Some(b) = a.get(j+1) {
            if *b <= input[i][j] {
                return false;
            }
        }
    }
    true
}

#[allow(unused_variables)]
fn part1(input: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            //
            let result = check_surroundings(i, j, input.clone());
            if result {
                debug!("i {} j {} val {} are low", i, j, input[i][j]);
                count += input[i][j]+1;
            }
        }
    }
    count
}

type Checked = Vec<(usize, usize)>;

fn check_basin_size<'a>(i: usize, j: usize, input: &Vec<Vec<u32>>, checked: &'a mut Checked) -> (u32, &'a mut Checked) {
    if checked.contains(&(i, j)) {
        return (0, checked);
    } else {
        checked.push((i, j));
    }
    trace!("check_basin_size {} {} {}", i, j, input[i][j]);
    let mut value = 1;
    if i > 0 {
        if let Some(a) = input.get(i-1) {
            if let Some(b) = a.get(j) {
                if *b != 9 && *b >= input[i][j] {
                    value += check_basin_size(i-1, j, input, checked).0;
                }
            }
        }
    }
    if let Some(a) = input.get(i+1) {
        if let Some(b) = a.get(j) {
            if *b != 9 && *b >= input[i][j] {
                value += check_basin_size(i+1, j, input, checked).0;
            }
        }
    }
    if j > 0 {
        if let Some(a) = input.get(i) {
            if let Some(b) = a.get(j-1) {
                if *b != 9 && *b >= input[i][j] {
                    value += check_basin_size(i, j-1, input, checked).0;
                }
            }
        }
    }
    if let Some(a) = input.get(i) {
        if let Some(b) = a.get(j+1) {
            if *b != 9 && *b >= input[i][j] {
                value += check_basin_size(i, j+1, input, checked).0;
            }
        }
    }
    (value, checked)
}

#[allow(unused_variables)]
fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut lows: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let result = check_surroundings(i, j, input.clone());
            if result {
                debug!("i {} j {} val {} are low", i, j, input[i][j]);
                lows.insert((i,j), 0);
            }
        }
    }
    for (k, v) in lows.clone() {
        let mut checked: Checked = vec![];
        let result = check_basin_size(k.0, k.1, &input, &mut checked);
        debug!("check_basin_size [i] {} [j] {} val {} result {}", k.0, k.1, input[k.0][k.1], result.0);
        if let Some(x) = lows.get_mut(&(k.0, k.1)) {
            *x = result.0;
        }
    }
    debug!("lows {:?}", lows);
    let mut highs: Vec<u32> = vec![0; 3];
    for (k, v) in lows.clone() {
        for i in 0..highs.len() {
            debug!("k {:?} v {} i {} highs {:?}", k, v, i, highs);
            if v > highs[i] {
                highs[i] = v;
                break;
            }
        }
        highs.sort();
    }
    debug!("highs {:?}", highs);
    let mut result = highs[0];
    for i in 1..highs.len() {
        result *= highs[i];
    }
    result
}
