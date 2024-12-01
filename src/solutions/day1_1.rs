use std::{error::Error, i32};

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
	
	let mut left: Vec<i32> = Vec::new();
	let mut right: Vec<i32> = Vec::new();

	for line in input.lines(){
		
		if line.trim().is_empty(){
			continue;
		}

		let mut numbers = line.split_whitespace();
		if let (Some(left_num), Some(right_num)) = (numbers.next(), numbers.next()){
			left.push(left_num.parse::<i32>()?);
			right.push(right_num.parse::<i32>()?);

		}
	}
	left.sort();
	right.sort();

	let mut distance: i64 = 0;
	//iterate through the left array and right array and calcuate the distance between the two numbers
	for (left_num, right_num) in left.iter().zip(right.iter()) {
        distance = distance + (left_num - right_num).abs() as i64;
    }


	return Ok(distance);
}
