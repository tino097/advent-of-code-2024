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

    let input4 = match utils::get_input(4){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day4_1::solve_part1(&input4){
        Ok(result) => println!("Day 4 Ceres Search: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day4_2::solve_part2(&input4){
        Ok(result) => println!("Day 4 Ceres Search X-MAS: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let input5 = match utils::get_input(5){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day5_1::solve_part1(&input5){
        Ok(result) => println!("Day 5 Print Queue: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day5_2::solve_part2(&input5){
        Ok(result) => println!("Day 5 Print Incorects Queue: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let input6 = match utils::get_input(6){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day6_1::solve_part1(&input6){
        Ok(result) => println!("Day 6 Guard Map: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // match solutions::day6_2::solve_part2(&input6){
    //     Ok(result) => println!("Day 6 Guard Map Loop: {}", result),
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // };

    let input7 = match utils::get_input(7){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day7_1::solve_part1(&input7){
        Ok(result) => println!("Day 7 Bridge Repair: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // match solutions::day7_2::solve_part2(&input7){
    //     Ok(result) => println!("Day 7 Bridge Repair Concatination: {}", result),
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // };

    let input8 = match utils::get_input(8){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day8_1::solve_part1(&input8){
        Ok(result) => println!("Day 8 Resonant Collienarity: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    };

    match solutions::day8_2::solve_part2(&input8){
        Ok(result) => println!("Day 8 Resonant: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }

    let input9 = match utils::get_input(9){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day9_1::solve_part1(&input9){
        Ok(result) => println!("Day 9 Checksum: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    };

    // match solutions::day9_2::solve_part2(&input9){
    //     Ok(result) => println!("Day 9 Checksum: {}", result),
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
        
    // }

    let input10 = match utils::get_input(10){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day10_1::solve_part1(&input10){
        Ok(result) => println!("Day 10 Trailheads: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }

    match solutions::day10_2::solve_part2(&input10){
        Ok(result) => println!("Day 10 Distinct Trailheads: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let input11 = match utils::get_input(11){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day11_1::solve_part1(&input11){
        Ok(result) => println!("Day 11 Stones Count: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }

    match solutions::day11_2::solve_part2(&input11){
        Ok(result) => println!("Day 11 Stones Count Plus: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }

    let input12 = match utils::get_input(12){
        Ok(input_str) => input_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match solutions::day12_1::solve_part1(&input12){
        Ok(result) => println!("Day 12 Region Price: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }

    match solutions::day12_2::solve_part2(&input12){
        Ok(result) => println!("Day 12 Region Price By Side: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        
    }
}