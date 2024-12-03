use std::error::Error;
use regex::Regex;


pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {

	//Parse the input to match exact string `mul(a, b)` where a and b are integers
	let pattern = r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)";
	let mut enabled: bool = true;
    let re = Regex::new(pattern).unwrap();

	let mut result: i64 = 0;

	// Do the multiplication only if regex matches `do()` and not `don't()`
	for cap in re.captures_iter(input){
		println!("{:?}", cap);		
		if cap.get(4).is_some(){
			enabled = true;
		}else if cap.get(5).is_some(){
			enabled = false;
		}else if cap.get(1).is_some() && enabled{
			println!("{:?}", enabled);
			let a = cap[2].parse::<i64>().unwrap();
			let b = cap[3].parse::<i64>().unwrap();
			result += a * b;
		}
	}
		
	Ok(result)
}