use std::error::Error;
use regex::Regex;


pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {

	//Parse the input to match exact string `mul(a, b)` where a and b are integers
	let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

	let result: i64 = re.captures_iter(input)
		.map(|cap| {
			let a = cap[1].parse::<i64>().unwrap();
			let b = cap[2].parse::<i64>().unwrap();
			a * b
		})
		.sum();
	
	Ok(result)
}