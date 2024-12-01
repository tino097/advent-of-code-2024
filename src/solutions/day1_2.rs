use std::{collections::HashMap, error::Error};


pub fn solve_part2(input:&str) -> Result<i64, Box<dyn Error>> {
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
	let right_count: HashMap<i32, i64> = right.iter()
		.fold(HashMap::new(), |mut map, &num| {
			*map.entry(num).or_insert(0) += 1;
			map
		});

	let similarity: i64 = left.iter()		
		.map(|&num| {
			let count = right_count.get(&num).unwrap_or(&0);
			(num as i64) * count
		}).sum();

	return Ok(similarity);
}