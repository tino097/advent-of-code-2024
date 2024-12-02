mod utils;
mod solutions;


fn main() {
    let input = match utils::get_input(2) {
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    match solutions::day1_1::solve_part1(&input){
        Ok(result) => println!("Day 1 Distance: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day1_2::solve_part2(&input){
        Ok(result) => println!("Day 1 Similarity: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day2_1::solve_part1(&input){
        Ok(result) => println!("Day 2 Safe Report: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day2_2::solve_part2(&input){
        Ok(result) => println!("Day 2 Safe Report: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}