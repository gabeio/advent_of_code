use std::io::{self};
use std::convert::TryFrom;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	match io::stdin().read_line(&mut buffer) {
		Ok(_) => {
			buffer.pop();
			// println!("{} bytes read", n);
			// println!("{}", buffer);
		}
		Err(error) => println!("error: {}", error),
	}
	let vstr: Vec<&str> = buffer.split(',').collect();
	// println!("{:?}", vstr);
	let convert = |x: &str| String::from(x).parse().unwrap();
	let mut vint: Vec<u32>;// = vstr.into_iter().map(convert).collect();
	// println!("{:?}", vint);
	'outeri: for i in 0..100 {
		'outerj: for j in 0..100 {
			// println!("i: {} j: {}", i,j);
			vint = vstr.to_vec().into_iter().map(convert).collect();
			vint[1] = i;
			vint[2] = j;
			// println!("i: {} j: {}", vint[1], vint[2]);
			'inner: for x in (0..vint.len()).step_by(4) {
				match opscode_execute(&mut vint, x, x+3) {
					Ok(_) => (),
					Err(v) => {
						if v > 0 {
							// println!("i: {} j: {} v: {}", i, j, v);
						}
						if v == 19690720 {
							println!("vint[0]: {}, noun: {}, verb: {}", vint[0], i, j);
							return Ok(());
						} else {
							continue 'outerj;
						}
					},
				}
			}
		}
	}
	Ok(())
}

fn opscode_execute(vint:&mut Vec<u32>, from:usize, to:usize) ->Result<(), u32> {
	// println!("from: {}, to: {}", from, to);
	if from > vint.len() {
		// println!("from > vint.len()");
		return Err(vint[0]);
	}
	let op = vint[from];
	if op == 99 {
		// println!("program halted normally");
		return Err(vint[0]);
	}
	if from+1 > vint.len() || from+2 > vint.len() || to > vint.len() {
		// println!("from+1 > vint.len()");
		return Err(vint[0]);
	}
	let ptr1:usize = usize::try_from(vint[from+1]).unwrap();
	let ptr2:usize = usize::try_from(vint[from+2]).unwrap();
	let dest:usize = usize::try_from(vint[to]).unwrap();
	if dest > vint.len() {
		// println!("dest > vint.len()");
		return Err(vint[0]);
	}
	// println!("op: {}, ptr1: {}, ptr2: {}, dest: {}", op, vint[ptr1], vint[ptr2], dest);
	if op == 1 {
		vint[dest] = vint[ptr1] + vint[ptr2];
	} else if op == 2 {
		vint[dest] = vint[ptr1] * vint[ptr2];
	}
	return Ok(());
}