mod utils;
mod solutions;


fn main() {
    let input1 = match utils::get_input(1){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    match solutions::day1_1::solve_part1(&input1){
        Ok(result) => println!("Day 1 Distance: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day1_2::solve_part2(&input1){
        Ok(result) => println!("Day 1 Similarity: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let input2 = match utils::get_input(2){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day2_1::solve_part1(&input2){
        Ok(result) => println!("Day 2 Safe Report: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day2_2::solve_part2(&input2){
        Ok(result) => println!("Day 2 Safe Report: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let input3 = match utils::get_input(3){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day3_1::solve_part1(&input3){
        Ok(result) => println!("Day 3 Multiplication: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day3_2::solve_part2(&input3){
        Ok(result) => println!("Day 3 Multiplication Do/Don't: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}