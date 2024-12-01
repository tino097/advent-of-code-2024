mod utils;
mod solutions;


fn main() {
    // Example usage
    let input = match utils::get_input(1) {
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    match solutions::day1_1::solve_part1(&input){
        Ok(result) => println!("Distance: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day1_2::solve_part2(&input){
        Ok(result) => println!("Similarity: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}