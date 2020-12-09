use std::io::{self, Read};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let mut plane: Plane = [[false; 8]; 128];
        assert_eq!(narrow_fb(plane, &"FBFBBFFRLR", 0, 127), (44, 5));
    }

    #[test]
    fn input2() {
        let mut plane: Plane = [[false; 8]; 128];
        assert_eq!(narrow_fb(plane, &"BFFFBBFRRR", 0, 127), (70, 7));
    }

    #[test]
    fn input3() {
        let mut plane: Plane = [[false; 8]; 128];
        assert_eq!(narrow_fb(plane, &"FFFBBBFRRR", 0, 127), (14, 7));
    }

    #[test]
    fn input4() {
        let mut plane: Plane = [[false; 8]; 128];
        assert_eq!(narrow_fb(plane, &"BBFFBBFRLL", 0, 127), (102, 4));
    }
}

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

type Plane = [[bool; 8]; 128];

fn part1(buffer: &String) {
    println!("Part 1 Beginning.");
    let vstr: Vec<&str> = buffer.split("\n").collect();
    let mut plane: Plane = [[false; 8]; 128];
    for x in vstr {
        let (_plane, seatId) = seat(plane, x);
        plane = _plane;
        // println!("Seat Id: {}", seatId);
    }
    let mut fb = 0;
    let mut lr = 0;
    for x in 0..127 {
        // println!("{:?}", plane[x]);
        for y in 0..7 {
            if plane[x][y] == true {
                if x > fb {
                    fb = x;
                    lr = 0;
                }
                if y > lr {
                    lr = y;
                }
            }
        }
    }
    println!("[{}:{}]", fb, lr);
    println!("Seat Id: {}", (fb * 8) + lr);
}

fn seat(mut plane: Plane, pass: &str) -> (Plane, usize) {
    let (row, column) = narrow_fb(plane, pass, 0, 127);
    plane[row as usize][column as usize] = true;
    (plane, (row * 8 + column))
}

fn narrow_fb(mut plane: Plane, pass: &str, front: usize, back: usize) -> (usize, usize) {
    //
    let count: usize = (back - front) / 2 + 1;
    let current = &pass[..1];
    let pass = &pass[1..];
    if count > 1 {
        if current == "F" {
            narrow_fb(plane, pass, front, back-count)
        } else {
            narrow_fb(plane, pass, front+count, back)
        }
    } else {
        if current == "F" {
            (front, narrow_lr(plane, pass, front, 0, 7))
        } else {
            (back, narrow_lr(plane, pass, back, 0, 7))
        }
    }
}

fn narrow_lr(mut plane: Plane, pass: &str, row: usize, left: usize, right: usize) -> usize {
    //
    let count: usize = (right - left) / 2 + 1;
    let current = &pass[..1];
    let pass = &pass[1..];
    if count > 1 {
        if current == "L" {
            narrow_lr(plane, pass, row, left, right-count)
        } else {
            narrow_lr(plane, pass, row, left+count, right)
        }
    } else {
        if current == "L" {
            left
        } else {
            right
        }
    }
}

fn part2(buffer: &String) {
    println!("Part 2 Beginning.");
    let vstr: Vec<&str> = buffer.split("\n").collect();
    let mut plane: Plane = [[false; 8]; 128];
    for x in vstr {
        let (_plane, seatId) = seat(plane, x);
        plane = _plane;
        // println!("Seat Id: {}", seatId);
    }
    let mut fb = 0;
    let mut lr = 0;
    for x in 0..127 {
        for y in 0..7 {
            if plane[x][y] == true {
                if x > fb {
                    fb = x;
                    lr = 0;
                }
                if y > lr {
                    lr = y;
                }
            }
        }
    }
    let mut in_a_row = 0;
    'outer: for x in 0..127 {
        for y in 0..7 {
            if in_a_row > 30 && plane[x][y] == false {
                println!("my seat [{}:{}]", x, y);
                println!("my seat Seat Id: {}", (x * 8) + y);
                break 'outer;
            }
            if plane[x][y] == true {
                in_a_row += 1;
            } else {
                in_a_row = 0;
            }
        }
    }
}