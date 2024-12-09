use std::error::Error;

#[derive(Debug)]
struct Equation {
   target: i64,
   numbers: Vec<i64>
}

#[derive(Debug, Clone, Copy)]
enum Operator {
   Add,
   Multiply,
   Concat
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
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

fn generate_operator_combinations(len: usize) -> Vec<Vec<Operator>> {
   let mut result = Vec::new();
   let operators = [Operator::Add, Operator::Multiply, Operator::Concat];
   
   // Generate all possible combinations with 3 operators
   for i in 0..3i64.pow(len as u32) {
       let mut combination = Vec::new();
       let mut n = i;
       
       for _ in 0..len {
           combination.push(operators[(n % 3) as usize]);
           n /= 3;
       }
       result.push(combination);
   }
   result
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
   let mut result = numbers[0];
   
   for i in 0..operators.len() {
       match operators[i] {
           Operator::Add => result += numbers[i + 1],
           Operator::Multiply => result *= numbers[i + 1],
           Operator::Concat => {
               // Convert both numbers to strings, concatenate, then parse back
               let concat = format!("{}{}", result, numbers[i + 1]);
               result = concat.parse().unwrap();
           }
       }
   }
   
   result
}