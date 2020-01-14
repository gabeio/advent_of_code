use std::io::{self};
// use std::convert::TryInto;

fn main() {
    let buffer: String = read_line(); // read first line
    let range: Vec<&str> = buffer.split('-').collect();
    println!("{:?}", range);
    find_valid_values(range);
}

fn read_line() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    buffer
}

fn find_valid_values(range: Vec<&str>) {
    let a: usize = str_to_usize(range[0]);
    let b: usize = str_to_usize(range[1]);
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    let mut count = 0;
    for number in a..=b {
        // Two adjacent digits are the same (like 22 in 122345).
        if !adjacent_digits(number) {
            continue;
        }
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        if !decreasing_digits(number) {
            continue;
        }
        println!("{}", number);
        count += 1;
    }
    println!("{}", count);
}

fn str_to_usize(i: &str) -> usize {
    String::from(i).parse().unwrap()
}

fn adjacent_digits(i: usize) -> bool {
    let mut j = i;
    let i0 = j % 10;
    j = j / 10;
    let i1 = j % 10;
    j = j / 10;
    let i2 = j % 10;
    j = j / 10;
    let i3 = j % 10;
    j = j / 10;
    let i4 = j % 10;
    j = j / 10;
    let i5 = j % 10;
    if i0 == i1 {
        if i0 == i2 || i0 == i3 || i0 == i4 || i0 == i5 {
            // pass
        } else {
            return true;
        }
    }
    if i1 == i2 {
        if i1 == i0 || i1 == i3 || i1 == i4 || i1 == i5 {
            // pass
        } else {
            return true;
        }
    }
    if i2 == i3 {
        if i2 == i0 || i2 == i1 || i2 == i4 || i2 == i5 {
            // pass
        } else {
            return true;
        }
    }
    if i3 == i4 {
        if i3 == i0 || i3 == i1 || i3 == i2 || i3 == i5 {
            // pass
        } else {
            return true;
        }
    }
    if i4 == i5 {
        if i4 == i0 || i4 == i1 || i4 == i2 || i4 == i3 {
            // pass
        } else {
            return true;
        }
    }
    return false;
}

fn decreasing_digits(i: usize) -> bool {
    let mut j = i;
    let i0 = j % 10;
    j = j / 10;
    let i1 = j % 10;
    j = j / 10;
    let i2 = j % 10;
    j = j / 10;
    let i3 = j % 10;
    j = j / 10;
    let i4 = j % 10;
    j = j / 10;
    let i5 = j % 10;
    if i5 <= i4 && i4 <= i3 && i3 <= i2 && i2 <= i1 && i1 <= i0
    {
        return true;
    }
    return false;
}



#[test]
fn test_adjacent_digits() {
    assert_eq!(adjacent_digits(122345 as usize), true);
}

#[test]
fn test_adjacent_digits_false() {
    assert_eq!(adjacent_digits(123789 as usize), false);
}

#[test]
fn test_decreasing_digits() {
    assert_eq!(decreasing_digits(123789 as usize), true);
}

#[test]
fn test_decreasing_digits_false() {
    assert_eq!(decreasing_digits(223450 as usize), false);
}

#[test]
fn test_adjacent_digits_part_two() {
    assert_eq!(adjacent_digits(111122 as usize), true);
}

#[test]
fn test_adjacent_digits_part_two_false() {
    assert_eq!(adjacent_digits(111111 as usize), false);
}

#[test]
fn test_both() {
    assert_eq!(adjacent_digits(112233 as usize), true);
    assert_eq!(decreasing_digits(112233 as usize), true);
}