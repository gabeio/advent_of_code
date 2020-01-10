use std::convert::TryInto;
use std::io::{self};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer: String = read_line(); // read first line
    buffer.pop(); // remove \n
    let wire1: Vec<&str> = buffer.split(',').collect();
    let buffer: String = read_line(); // read second line
    let wire2: Vec<&str> = buffer.split(',').collect();
    follow_wires(wire1, wire2);
    Ok(())
}

fn read_line() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    buffer
}

fn follow_wires<'a>(wire1: Vec<&str>, wire2: Vec<&str>) {
    let grid: HashMap<(i32, i32),i32> = HashMap::new();
    let grid: HashMap<(i32, i32),i32> = walk_wire(wire1, grid, 1);
    let grid: HashMap<(i32, i32),i32> = walk_wire(wire2, grid, 3);
    find_closest(&grid);
}

fn walk_wire(wire: Vec<&str>, grid: HashMap<(i32, i32),i32>, add: i32) -> HashMap<(i32, i32),i32> {
    let convert = |i: &str| String::from(remove_first(i)).parse().unwrap();
    let mut grid: HashMap<(i32, i32),i32> = grid;
    let mut cursor: Vec<i32> = vec![0, 0];
    for i in wire {
        let distance: i32 = convert(i);
        let direction: char = i.chars().nth(0).unwrap().try_into().unwrap();
        let (_grid, _cursor) = walk(cursor, direction, distance, grid, add);
        grid = _grid;
        cursor = _cursor;
    }
    grid
}

fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

fn walk(
    mut cursor: Vec<i32>,
    direction: char,
    distance: i32,
    mut grid: HashMap<(i32, i32),i32>,
    add: i32,
) -> (HashMap<(i32, i32),i32>, Vec<i32>) {
    let mut x = cursor[0];
    let mut y = cursor[1];
    for _distance in 0..distance {
        if direction == 'R' {
            x += 1;
            cursor[0] += 1;
            grid.entry((x, y)).and_modify(|e| { *e += add }).or_insert(add);
        } else if direction == 'L' {
            x -= 1;
            cursor[0] -= 1;
            grid.entry((x, y)).and_modify(|e| { *e += add }).or_insert(add);
        } else if direction == 'U' {
            y += 1;
            cursor[1] += 1;
            grid.entry((x, y)).and_modify(|e| { *e += add }).or_insert(add);
        } else if direction == 'D' {
            y -= 1;
            cursor[1] -= 1;
            grid.entry((x, y)).and_modify(|e| { *e += add }).or_insert(add);
        } else {
            panic!("Unknown {:?}", distance);
        }
    }
    return (grid, cursor);
}

fn find_closest(grid: &HashMap<(i32, i32),i32>) {
    let mut closest = std::i32::MAX;
    for (key, value) in grid.iter() {
        if value == &4i32 || value == &5i32 || value == &7i32 || value == &8i32 {
            if key.0.abs() + key.1.abs() < closest {
                closest = key.0.abs() + key.1.abs();
            }
        }
    }
    println!("closest: {}", closest);
}

#[derive(Clone)]
struct Closest {
    x: i32,
    y: i32,
    distance: i32,
}
