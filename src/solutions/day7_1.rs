use std::error::Error;

#[derive(Debug)]
struct Equation {
    target: i64,
    numbers: Vec<i64>
}

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;

    for line in input.lines() {
        let equation = parse_equation(line);
        let operator_count = equation.numbers.len() - 1;
        let operator_combinations = generate_operator_combinations(operator_count);

        // Check if any combination produces the target value
        let is_valid = operator_combinations.iter().any(|operators| {
            evaluate(&equation.numbers, operators) == equation.target
        });

        if is_valid {
            sum += equation.target;
        }
    }

    Ok(sum)
}

fn parse_equation(line: &str) -> Equation {
    let (target, rest) = line.split_once(':').unwrap();
    Equation {
        target: target.trim().parse().unwrap(),
        numbers: rest.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
    }
}

fn generate_operator_combinations(len: usize) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let total_combinations = 1 << len;
    
    for i in 0..total_combinations {
        let mut combination = Vec::new();
        for j in 0..len {
            let operator = if (i & (1 << j)) != 0 { '+' } else { '*' };
            combination.push(operator);
        }
        result.push(combination);
    }
    result
}

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => panic!("Invalid operator"),
        }
    }
    
    result
}