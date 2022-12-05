#[macro_use]
extern crate log;

use std::io::{self, Read};

// grab input as solid string
pub fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    let buffer = buffer.trim();
    return buffer.to_string();
}

trait AOC {
    fn int_grid(&self, split: char) -> Vec<Vec<u32>>;
    fn string_grid(buffer: &String, split: char) -> Vec<Vec<String>>;
    fn u8_grid(buffer: &String, split: char) -> Vec<Vec<u8>>;
    fn int_list(buffer: &String, split: char) -> Vec<u32>;
    fn str_list(buffer: &String, split: char) -> Vec<&str>;
    fn string_list(buffer: &String, split: char) -> Vec<String>;
}

impl AOC for &String {
    // split lines
    // split lines by character
    // parse characters into ints
    // collect into grid
    fn int_grid(&self, split: char) -> Vec<Vec<u32>> {
        // convert buffer to lines
        let vstr: Vec<&str> = self.split(split).collect();
        trace!("vstr {:?}", &vstr);
        let vstr: Vec<Vec<char>> = vstr.iter().map(|e| e.chars().collect()).collect();
        let convert = |x: char| String::from(x).parse().unwrap();
        let vconvert = |x: Vec<char>| x.clone().into_iter().map(convert).collect();
        let vint: Vec<Vec<u32>> = vstr.clone().into_iter().map(vconvert).collect();
        trace!("vint {:?}", &vint);
        vint
    }

    // split lines
    // split lines by character
    // collect into grid
    fn string_grid(buffer: &String, split: char) -> Vec<Vec<String>> {
        let vstr: Vec<&str> = buffer.split(split).collect();
        trace!("vstr {:?}", &vstr);
        let vstr: Vec<Vec<char>> = vstr.iter().map(|e| e.chars().collect()).collect();
        let convert = |x: char| String::from(x);
        let vconvert = |x: Vec<char>| x.clone().into_iter().map(convert).collect();
        let vstr: Vec<Vec<String>> = vstr.clone().into_iter().map(vconvert).collect();
        vstr
    }

    // split lines
    fn u8_grid(buffer: &String, split: char) -> Vec<Vec<u8>> {
        let vstr: Vec<&str> = buffer.split(split).collect();
        trace!("vstr {:?}", &vstr);
        let vvu8: Vec<Vec<u8>> = vstr.iter().map(|e| e.as_bytes().to_vec()).collect();
        trace!("vstr {:?}", &vvu8);
        vvu8
    }

    // split lines
    // convert to ints
    fn int_list(buffer: &String, split: char) -> Vec<u32> {
        let vstr: Vec<&str> = buffer.split(split).collect();
        trace!("vstr {:?}", &vstr);
        let convert = |x: &str| String::from(x).parse().unwrap();
        let vint: Vec<u32> = vstr.clone().into_iter().map(convert).collect();
        trace!("vint {:?}", &vint);
        vint
    }

    // split lines
    fn str_list(buffer: &String, split: char) -> Vec<&str> {
        let vstr: Vec<&str> = buffer.split(split).collect();
        trace!("vstr {:?}", &vstr);
        vstr
    }

    // split lines
    fn string_list(buffer: &String, split: char) -> Vec<String> {
        let vstr: Vec<&str> = buffer.split(split).collect();
        trace!("vstr {:?}", &vstr);
        let convert = |x: &str| String::from(x);
        let vstring: Vec<String> = vstr.clone().into_iter().map(convert).collect();
        vstring
    }
}

#[cfg(test)]
mod tests {
    use super;
    #[test]
    fn readin() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
