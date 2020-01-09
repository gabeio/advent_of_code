use std::convert::TryInto;
use std::io::{self};

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
    let wire1_max: Vec<i32> = find_max_distance(&wire1);
    let wire2_max: Vec<i32> = find_max_distance(&wire2);
    let maxs: Vec<i32> = max_distances(wire1_max, wire2_max);
    let grid_size_x: usize = (maxs[0] + maxs[1] + 2).try_into().unwrap();
    let grid_size_y: usize = (maxs[2] + maxs[3] + 2).try_into().unwrap();
    let arr = vec![0; grid_size_x];
    let grid: Vec<Vec<i32>> = vec![arr; grid_size_y];
    let grid: Vec<Vec<i32>> = walk_wire(wire1, grid, &maxs, 1);
    let grid: Vec<Vec<i32>> = walk_wire(wire2, grid, &maxs, 3);
    let pgrid: Grid = Grid {
        yx: &grid,
    };
    println!("{}", pgrid);
    find_closest(&maxs, &grid, 4); // 1 + 3
    find_closest(&maxs, &grid, 5); // 1 + 1 + 3
    find_closest(&maxs, &grid, 7); // 1 + 3 + 3
    find_closest(&maxs, &grid, 8); // 1 + 1 + 3 + 3
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
    vec![left.abs(), right.abs(), up.abs(), down.abs()]
}

fn max_distances(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let left: i32 = if a[0] > b[0] { a[0] } else { b[0] };

    let right: i32 = if a[1] > b[1] { a[1] } else { b[1] };

    let up: i32 = if a[2] > b[2] { a[2] } else { b[2] };

    let down: i32 = if a[3] > b[3] { a[3] } else { b[3] };

    vec![left, right, up, down]
}

fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

fn walk_wire(wire: Vec<&str>, grid: Vec<Vec<i32>>, maxs: &Vec<i32>, add: i32) -> Vec<Vec<i32>> {
    let convert = |i: &str| String::from(remove_first(i)).parse().unwrap();
    let mut grid: Vec<Vec<i32>> = grid;
    let mut origin: Vec<i32> = vec![maxs[3] + 1, maxs[0] + 1];
    let y: usize = origin[0].try_into().unwrap();
    let x: usize = origin[1].try_into().unwrap();
    grid[y][x] = 0;
    for i in wire {
        let distance: i32 = convert(i);
        let direction: char = i.chars().nth(0).unwrap().try_into().unwrap();
        let (_grid, _origin) = walk(origin, direction, distance, grid, add);
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
    add: i32,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    for _distance in 0..distance {
        if direction == 'R' {
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += add;
            origin[1] += 1;
        } else if direction == 'L' {
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += add;
            origin[1] -= 1
        } else if direction == 'U' {
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += add;
            origin[0] += 1;
        } else if direction == 'D' {
            let y: usize = origin[0].try_into().unwrap();
            let x: usize = origin[1].try_into().unwrap();
            grid[y][x] += add;
            origin[0] -= 1;
        } else {
            panic!("Unknown {:?}", distance);
        }
    }
    return (grid, origin);
}

fn find_closest(maxs: &Vec<i32>, grid: &Vec<Vec<i32>>, value: i32) {
    let origin: Vec<i32> = vec![maxs[3] + 1, maxs[0] + 1];
    let origin_sum: usize = (origin[0] + origin[1]).try_into().unwrap();
    let mut closest: Vec<Closest> = vec![Closest {
        x: 0,
        y: 0,
        distance: std::i32::MAX,
    }; 4];
    let y: usize = origin[0] as usize;
    let x: usize = origin[1] as usize;
    println!("origin: {} {}", x, y);
    // q1
    for i in y..grid.len() {
        for j in x..grid[0].len() {
            if grid[i][j] == value {
                println!("({},{}) {}", i, j, grid[i][j]);
                if closest[0].distance > (((i + j) - origin_sum) as i32) {
                    closest[0] = Closest {
                        y: (i) as i32,
                        x: (j) as i32,
                        distance: ((i + j) - origin_sum) as i32,
                    }
                }
            }
        }
    }
    // q3
    for a in (0..=y).rev() {
        for b in (0..=x).rev() {
            if grid[a][b] == value {
                println!("({},{}) {}", a, b, grid[a][b]);
                if closest[2].distance > ((a + b) - origin_sum) as i32 {
                    closest[2] = Closest {
                        y: (a) as i32,
                        x: (b) as i32,
                        distance: ((a + b) - origin_sum) as i32,
                    }
                }
            }
        }
    }
    // q2
    for c in (0..=y).rev() {
        for d in x..grid[0].len() {
            if grid[c][d] == value {
                println!("({},{}) {}", c, d, grid[c][d]);
                if closest[1].distance > ((c + d) - origin_sum) as i32 {
                    closest[1] = Closest {
                        y: (c) as i32,
                        x: (d) as i32,
                        distance: ((c + d) - origin_sum) as i32,
                    }
                }
            }
        }
    }
    // q4
    for e in y..grid.len() {
        for f in (0..=x).rev() {
            if grid[e][f] == value {
                println!("({},{}) {}", e, f, grid[e][f]);
                if closest[4].distance > ((e + f) - origin_sum) as i32 {
                    closest[4] = Closest {
                        y: (e) as i32,
                        x: (f) as i32,
                        distance: ((e + f) - origin_sum) as i32,
                    }
                }
            }
        }
    }
    if closest[0].distance != std::i32::MAX {
        println!(
            "closest: x: {} y: {} distance: {}",
            closest[0].x, closest[0].y, closest[0].distance
        );
    }
    if closest[1].distance != std::i32::MAX {
        println!(
            "closest: x: {} y: {} distance: {}",
            closest[1].x, closest[1].y, closest[1].distance
        );
    }
    if closest[2].distance != std::i32::MAX {
        println!(
            "closest: x: {} y: {} distance: {}",
            closest[2].x, closest[2].y, closest[2].distance
        );
    }
    if closest[3].distance != std::i32::MAX {
        println!(
            "closest: x: {} y: {} distance: {}",
            closest[3].x, closest[3].y, closest[3].distance
        );
    }
}

#[derive(Clone)]
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
