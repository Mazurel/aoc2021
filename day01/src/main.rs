use std::fs;
use std::str;

fn part1(filename: &str) -> std::result::Result<i32, String> {
    match fs::read_to_string(filename) {
	Err(err) => {
	    Err(err.to_string())
	},
	Ok(contents) => {
	    let mut counter = 0;
	    let mut prev = None;
	    for line in contents.lines() {
		if let Ok(number) = line.parse::<i32>() {
		    match prev {
			None => prev = Some(number),
			Some(unwrapped_prev) => {
			    if number > unwrapped_prev {
				counter = counter + 1;
			    }
			    prev = Some(number)
			}
		    }
		}
	    }
	    Ok(counter)
	}
    }  
}

fn part2(filename: &str) -> std::result::Result<i32, String> {
    match fs::read_to_string(filename) {
	Err(err) => {
	    Err(err.to_string())
	},
	Ok(contents) => {
	    let mut counter = 0;
	    	    
	    let mut A = None;
	    let mut B = None;
	    let mut C = None;
	    let mut D = None;
	    
	    for (index, line) in contents.lines().enumerate() {
		// println!("{:?}", (A, B, C, D));
		if let Ok(number) = line.parse::<i32>() {
		    match (A, B, C, D) {
			(None, None, None, None) => {
			    A = Some(number);
			},
			(Some(An), None, None, None) => {
			    A = Some(An + number);
			    B = Some(number);
			},
			(Some(An), Some(Bn), None, None) => {
			    A = Some(An + number);
			    B = Some(Bn + number);
			    C = Some(number);
			},
			(Some(An), Some(Bn), Some(Cn), None) => {
			    let tmp = Bn + number;

			    A = None;
			    B = Some(tmp);
			    C = Some(Cn + number);
			    D = Some(number);
			    
			    if An < tmp {
				counter = counter + 1;
			    }
			},
			(None, Some(Bn), Some(Cn), Some(Dn)) => {
			    let tmp = Cn + number;

			    A = Some(number);
			    B = None;
			    C = Some(tmp);
			    D = Some(Dn + number);

			    if Bn < tmp {
				counter = counter + 1;
			    }
			},
			(Some(An), None, Some(Cn), Some(Dn)) => {
			    let tmp = Dn + number;

			    A = Some(An + number);
			    B = Some(number);
			    C = None;
			    D = Some(tmp);

			    if Cn < tmp {
				counter = counter + 1;
			    }
			},
			(Some(An), Some(Bn), None, Some(Dn)) => {
			    let tmp = An + number;

			    A = Some(tmp);
			    B = Some(Bn + number);
			    C = Some(number);
			    D = None;

			    if Dn < tmp {
				counter = counter + 1;
			    }
			},
			_ => {}
		    }
		}
	    }
	    Ok(counter)
	}
    }  
}

fn main() {
    let filename = "input.txt";

    match part2(filename) {
	Ok(number) => println!("Number = {}", number),
	Err(message) => print!("Error: {}", message)
    }
}

