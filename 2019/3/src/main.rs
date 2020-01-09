use std::convert::TryInto;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut buffer: String = read_line(); // read first line
    buffer.pop(); // remove \n
    let wire1: Vec<&str> = buffer.split(',').collect();
    let buffer: String = read_line(); // read second line
    let wire2: Vec<&str> = buffer.split(',').collect();
    // println!("wire1: {:?}", wire1);
    // println!("wire2: {:?}", wire2);
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
    // let grid_size: i32 = (find_max(&wire1, &wire2) + 2)/2 + 196;
    let wire1_max: Vec<i32> = find_max_distance(&wire1);
    // println!("wire1 maxs: {:?}", wire1_max);
    let wire2_max: Vec<i32> = find_max_distance(&wire2);
    // println!("wire2 maxs: {:?}", wire2_max);
    let maxs: Vec<i32> = max_distances(wire1_max, wire2_max);
    let grid_size_x: usize = (maxs[0] + maxs[1] + 2).try_into().unwrap();
    let grid_size_y: usize = (maxs[2] + maxs[3] + 2).try_into().unwrap();
    // println!("grid_size_x: {}", grid_size_x);
    // println!("grid_size_y: {}", grid_size_y);
    let arr = vec![0; grid_size_x];
    let grid: Vec<Vec<i32>> = vec![arr; grid_size_y];

    let grid: Vec<Vec<i32>> = walk_wire(wire1, grid, &maxs);
    let grid: Vec<Vec<i32>> = walk_wire(wire2, grid, &maxs);
    let pgrid: Grid = Grid {
        yx: &grid,
    };
    // let grid: Grid = walk_wire(wire2, grid);
    // let grid = minimize_grid(grid);
    println!("{}", pgrid);
    find_closest(&maxs, &grid);
}

fn find_max_distance(wire: &Vec<&str>) -> Vec<i32> {
    let convert = |i: &str| String::from(remove_first(i)).parse().unwrap();
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut up: i32 = 0;
    let mut down: i32 = 0;
    let mut left_right: i32 = 0;
    let mut up_down: i32 = 0;
    for i in wire {
        let distance: i32 = convert(i);
        let direction: char = i.chars().nth(0).unwrap().try_into().unwrap();
        // println!("FMD: dist: {} dir: {}", distance, direction);
        if direction == 'R' {
            left_right += distance;
        } else if direction == 'L' {
            left_right -= distance;
        } else if direction == 'U' {
            up_down += distance;
        } else if direction == 'D' {
            up_down -= distance;
        } else {
            panic!("Unknown {:?}", distance);
        }
        let y = up_down;
        let x = left_right;
        if y > up {
            up = y;
        }
        if y < down {
            down = y;
        }
        if x > right {
            right = x;
        }
        if x < left {
            left = x;
        }
    }
    // println!("fmd: left {} right {} up {} down {}", left,right,up,down);
    vec![left.abs(), right.abs(), up.abs(), down.abs()]
}

fn max_distances(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let left: i32 = if a[0] > b[0] { a[0] } else { b[0] };

    let right: i32 = if a[1] > b[1] { a[1] } else { b[1] };

    let up: i32 = if a[2] > b[2] { a[2] } else { b[2] };

    let down: i32 = if a[3] > b[3] { a[3] } else { b[3] };

    // println!("md: left {} right {} up {} down {}", left,right,up,down);
    vec![left, right, up, down]
}

fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

fn walk_wire(wire: Vec<&str>, grid: Vec<Vec<i32>>, maxs: &Vec<i32>) -> Vec<Vec<i32>> {
    let convert = |i: &str| String::from(remove_first(i)).parse().unwrap();
    let mut grid: Vec<Vec<i32>> = grid;
    // let mut origin: Vec<i32> = vec!((grid.len()/2) as i32,(grid.len()/2) as i32);
    let mut origin: Vec<i32> = vec![maxs[3] + 1, maxs[0] + 1];
    let y: usize = origin[0].try_into().unwrap();
    let x: usize = origin[1].try_into().unwrap();
    grid[y][x] = 0;
    // println!("origin: {:?}", origin);
    for i in wire {
        let distance: i32 = convert(i);
        let direction: char = i.chars().nth(0).unwrap().try_into().unwrap();
        // println!("walk_wire: {:?} {} {}", origin, direction, distance);
        let (_grid, _origin) = walk(origin, direction, distance, grid);
        grid = _grid;
        origin = _origin;
    }
    grid
}

fn walk(
    mut origin: Vec<i32>,
    direction: char,
    distance: i32,
    mut grid: Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    for _distance in 0..distance {
        if direction == 'R' {
            // println!("usize: [{:?}][{:?}]", origin[0], origin[1]);
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += 1;
            origin[1] += 1;
        } else if direction == 'L' {
            // println!("usize: [{:?}][{:?}]", origin[0], origin[1]);
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += 1;
            origin[1] -= 1
        } else if direction == 'U' {
            // println!("usize: [{:?}][{:?}]", origin[0], origin[1]);
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += 1;
            origin[0] += 1;
        } else if direction == 'D' {
            // println!("usize: [{:?}][{:?}]", origin[0], origin[1]);
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += 1;
            origin[0] -= 1;
        } else {
            panic!("Unknown {:?}", distance);
        }
    }
    return (grid, origin);
}

fn find_closest(maxs: &Vec<i32>, grid: &Vec<Vec<i32>>) {
    let mut origin: Vec<i32> = vec![maxs[3] + 1, maxs[0] + 1];
    let mut closest_up = Closest {
        x: 0,
        y: 0,
        distance: std::i32::MAX,
    };
    let y: usize = origin[0].try_into().unwrap();
    let x: usize = origin[1].try_into().unwrap();
    println!("origin: {} {}", x, y);
    for i in y..grid.len() {
        for j in x..grid[i].len() {
            // look for closest 2+
            if grid[i][j] > 1 {
                println!("({},{}) {}",i,j,grid[i][j]);
                if closest_up.distance > ((i + j - 2) as i32) {
                    closest_up = Closest {
                        y: (i) as i32,
                        x: (j) as i32,
                        distance: (i + j -2) as i32,
                    }
                }
            }
        }
    }
    let current_distance: i32 = closest_up.distance;
    let mut closest_down = Closest {
        x: 0,
        y: 0,
        distance: std::i32::MAX,
    };
    for a in (0..=y).rev() {
        for b in (0..=x).rev() {
            // look for closest 2+
            if grid[a][b] > 1 {
                if closest_down.distance > (a + b - 2) as i32 {
                    closest_down = Closest {
                        y: (a) as i32,
                        x: (b) as i32,
                        distance: (a + b - 2) as i32,
                    }
                }
            }
        }
    }
    println!(
        "closest: x: {} y: {} distance: {}",
        closest_up.x, closest_up.y, closest_up.distance
    );
    println!(
        "closest: x: {} y: {} distance: {}",
        closest_down.x, closest_down.y, closest_down.distance
    );
}

struct Closest {
    x: i32,
    y: i32,
    distance: i32,
}

struct Grid<'a> {
    yx: &'a Vec<Vec<i32>>,
}

impl<'a> std::fmt::Display for Grid<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in (0..self.yx.len()).rev() {
            for j in 0..self.yx[i].len() {
                if self.yx[i][j] == 0 {
                    write!(f, ".").unwrap();
                } else {
                    write!(f, "{}", self.yx[i][j]).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}
