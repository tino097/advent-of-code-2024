use std::error::Error;

pub fn solve_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut safe_report: i32 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let numbers = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&numbers) {
            safe_report += 1;
        }
    }
    return Ok(safe_report);
}

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let first_diff = numbers[1] - numbers[0];

    if first_diff == 0 {
        return false;
    }

	let increase = first_diff > 0;

	for i in 0..numbers.len()-1 {
		let diff = numbers[i+1] - numbers[i];
		if diff.abs() < 1 || diff.abs() > 3 {
			return false;
		} 
		if increase && diff < 0 || !increase && diff > 0 {
			return false;
		}
	}
	return true;
}
