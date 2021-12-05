use std::fs;
use std::vec;

fn part1(filename: &str) -> Result<i32, String> {
	match fs::read_to_string(filename) {
		Err(err) => Err(err.to_string()),
		Ok(contents) => {
			let line_length = {
				let first_line = contents.lines().next().unwrap();

				first_line.len()
			};

			// > 0 -> 1
			// < 0 -> 0
			let mut bits = vec![0; line_length];

			for line in contents.lines() {
				for (index, bit) in line.as_bytes().iter().enumerate() {
					match *bit as char {
						'0' => bits[index] = bits[index] - 1,
						'1' => bits[index] = bits[index] + 1,
						_ => {}
					}
				}
			}

			let mut gamma = 0;
			let mut epsilon = 0;

			for (index, bit_number) in bits.iter().rev().enumerate() {
				match *bit_number > 0 {
					true => {
						gamma += (2 as i32).pow(index as u32);
					}
					false => {
						epsilon += (2 as i32).pow(index as u32);
					}
				}
			}

			Ok(gamma * epsilon)
		}
	}
}

fn binary_to_decimal(bits: &str) -> i32 {
	bits.as_bytes()
		.iter()
		.rev()
		.enumerate()
		.map(|(index, bit_number)| {
			if *bit_number as char == '1' {
				(2 as i32).pow(index as u32)
			} else {
				0
			}
		})
		.sum()
}

fn calculate_bit_ratio(index: usize, numbers: &Vec<&str>) -> i32 {
	numbers
		.iter()
		.map(|number| match number.as_bytes()[index] as char {
			'0' => -1,
			'1' => 1,
			_ => 0,
		})
		.sum()
}

fn part2(filename: &str) -> Result<i32, String> {
	match fs::read_to_string(filename) {
		Err(err) => Err(err.to_string()),
		Ok(contents) => {
			let oxygen_generator_rating = {
				let mut lines: Vec<&str> = contents.lines().collect();
				let mut index = 0;

				loop {
					let ratio = calculate_bit_ratio(index, &lines);
					if ratio >= 0 {
						lines = lines
							.iter()
							.cloned()
							.filter(|number| number.as_bytes()[index] as char == '1')
							.collect::<Vec<&str>>();
					} else {
						lines = lines
							.iter()
							.cloned()
							.filter(|number| number.as_bytes()[index] as char == '0')
							.collect::<Vec<&str>>();
					}

					if lines.len() == 1 {
						break;
					}
					index += 1;
				}

				binary_to_decimal(*lines.first().unwrap())
			};

			let co2_scrubber_rating = {
				let mut lines: Vec<&str> = contents.lines().collect();
				let mut index = 0;

				loop {
					let ratio = calculate_bit_ratio(index, &lines);
					if ratio >= 0 {
						lines = lines
							.iter()
							.cloned()
							.filter(|number| number.as_bytes()[index] as char == '0')
							.collect::<Vec<&str>>();
					} else {
						lines = lines
							.iter()
							.cloned()
							.filter(|number| number.as_bytes()[index] as char == '1')
							.collect::<Vec<&str>>();
					}

					if lines.len() == 1 {
						break;
					}
					index += 1;
				}

				binary_to_decimal(*lines.first().unwrap())
			};

			Ok(co2_scrubber_rating * oxygen_generator_rating)
		}
	}
}

fn main() {
	let filename = "input.txt";

	println!("{:?}", part2(filename));
}
