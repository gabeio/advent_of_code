use std::io::{self, Read};

fn main() -> io::Result<()> {
    let buffer = readin();
    part1(&buffer);
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
