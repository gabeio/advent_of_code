#[macro_use]
extern crate log;
extern crate aoc;

#[allow(unused_imports)]
use aoc::{
    readin,
    AOC,
};

#[allow(unused_imports)]
use std::collections::BTreeMap;

fn main() {
    env_logger::init();
    let buffer = readin();
    trace!("buffer {:?}", &buffer);
    let result = buffer.int_grid('\n');
    println!("part 1: {:?}", part1(&result));
    println!("part 2: {:?}", part2(&result));
}

fn plus1(board: &mut Vec<Vec<u32>>) {
    // 1. the energy level of each octopus increases by 1
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j] += 1;
        }
    }
}

fn step(board: &mut Vec<Vec<u32>>) -> u32 {
    // 2. any octopus with an energy level greater than 9 flashes.
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.
    // If this causes an octopus to have an energy level greater than 9,
    // it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
    let mut flashes = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            flashes += flash(&mut board, i, j);
        }
    }
    // 3. any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    flashes
}

fn flash(board: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut flashes = 0;
    if board[i][j] >= 10 {
        // flash
        flashes += 1;
        let im1: bool = i != 0;
        let jm1: bool = j != 0;
        if im1 {
            match board.get_mut(i-1) {
                Some(z) => { 
                    if jm1 {
                        match z.get_mut(j-1) {
                            Some(x) => *x += 1,
                            None => panic!("get_mut(j) was none"),
                        }
                    }
                    if let Some(x) = z.get_mut(j) {
                        *x += 1;
                    }
                    if let Some(x) = z.get_mut(j+1) {
                        *x += 1;
                    }
                },
                None => panic!("get_mut(i) was none"),
            }
        }
        if let Some(z) = board.get_mut(i) {
            if jm1 {
                match z.get_mut(j-1) {
                    Some(x) => *x += 1,
                    None => panic!("get_mut(j) was none"),
                }
            }
            if let Some(x) = z.get_mut(j) {
                *x += 1;
            }
            if let Some(x) = z.get_mut(j+1) {
                *x += 1;
            }
        }
        if let Some(z) = board.get_mut(i+1) {
            if jm1 {
                match z.get_mut(j-1) {
                    Some(x) => *x += 1,
                    None => panic!("get_mut(j) was none"),
                }
            }
            if let Some(x) = z.get_mut(j) {
                *x += 1;
            }
            if let Some(x) = z.get_mut(j+1) {
                *x += 1;
            }
        }
        board[i][j] = 0;
    }
    flashes
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut input: Vec<Vec<u32>> = input.clone();
    let mut total = 0;
    draw_board(&input);
    for i in 0..100 {
        plus1(&mut input);
        let steps = step(&mut input);
        total += steps;
        debug!("after step {}: {}", i, steps);
        draw_board(&input);
    }
    total
}

fn draw_board(input: &Vec<Vec<u32>>) {
    for y in 0..=9 {
        for x in 0..=9 {
            print!("{},", input[y][x]);
        }
        print!("\n");
    }
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    0
}
