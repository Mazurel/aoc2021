use std::fs;
use std::vec;

fn part1(filename: &str) -> Result<i32, String> {
    match fs::read_to_string(filename)
    {
	Err(err) => Err(err.to_string()),
	Ok(contents) => {
	    let mut pos = 0;
	    let mut depth = 0;
	
	    for line in contents.lines() {
		let mut splitted = line.split(' ');

		let command = splitted.next().unwrap();
		let number = splitted.next().unwrap().parse::<i32>().unwrap();
		
		match command {
		    "forward" => {
			pos = pos + number;
		    },
		    "down" => {
			depth = depth + number;
		    },
		    "up" => {
			depth = depth - number;
		    },
		    _ => {}
		}
	    }

	    Ok(pos * depth)
	}
    }
}

fn part2(filename: &str) -> Result<i32, String> {
    match fs::read_to_string(filename)
    {
	Err(err) => Err(err.to_string()),
	Ok(contents) => {
	    let mut pos = 0;
	    let mut depth = 0;
	    let mut aim = 0;

	    for line in contents.lines() {
		let mut splitted = line.split(' ');

		let command = splitted.next().unwrap();
		let number = splitted.next().unwrap().parse::<i32>().unwrap();
		
		match command {
		    "forward" => {
			pos = pos + number;
			depth = depth + aim * number;
		    },
		    "down" => {
			aim = aim + number;
		    },
		    "up" => {
			aim = aim - number;
		    },
		    _ => {}
		}
	    }

	    println!("pos: {}, depth: {}", pos, depth);
	    
	    Ok(pos * depth)
	}
    }
}

fn main() {
	let filename = "input.txt";

	println!("{:?}", part2(filename));
}
